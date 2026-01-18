use crate::{
    WriteMode,
    pokemon::{PokedexColor, Pokemon, PokemonType, StatWithOrder},
};
use memmap2::Mmap;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    // fs::File,
    fs::File,
    io::{self, BufRead, BufWriter, Write},
};
use miniserde::json;

pub type SingleSearchReturn = Option<Pokemon>;
pub type MultiSearchReturn = Vec<Pokemon>;
pub enum PokedexSearchResualt {
    Single(SingleSearchReturn),
    Multi(MultiSearchReturn),
}
impl PokedexSearchResualt {
    pub fn to_vec(&self) -> Vec<Pokemon> {
        match self {
            PokedexSearchResualt::Single(single) => match single {
                Some(pkmn) => Vec::from([pkmn.clone()]),
                None => Vec::new(),
            },
            PokedexSearchResualt::Multi(vec) => {
                vec.clone()
                // for pkmn in vec {
                //     pkmn.print(detail_level)
                // }
                // if vec.is_empty() {
                //     println!("sorry we couldnt find any thing")
                // }
            }
        }
    }
    pub fn print_data(&self, detail_level: u8) {
        let vec = self.to_vec();
        if vec.is_empty() {
            println!("sorry we couldnt find any thing");
            return;
        }
        for pokemon in self.to_vec() {
            pokemon.print(detail_level);
        }
    }
    pub fn write_data_to_file(
        &self,
        fp: String,
        detail_level: u8,
        write_mode: WriteMode,
    ) -> io::Result<()> {
        println!("writing to {}", fp);

        let file = File::create(fp)
            .unwrap_or_else(|e| panic!("sorry rsdex could not create your file because {e}"));

        // let pokemon:&[u8] = &self.to_vec().iter().map(|pkmn|pkmn.get_data_as_string(detail_level)).map(|s|s.as_bytes()).flatten().copied().collect::<Vec<u8>>();

        let mut writer = BufWriter::new(file);
        let vec = self.to_vec();

        match write_mode {
            WriteMode::Json => {
                writer.write_all("[".as_bytes())?;
                for (i, data) in vec.iter().enumerate() {
                    writer.write_all(
                        json::to_string(&data.get_data_as_string(detail_level))
                            .as_bytes(),
                    )?;
                    if i < vec.len() - 1 {
                        writer.write_all(",\n".as_bytes())?;
                    }
                }
                writer.write_all("\n]".as_bytes())?;
            }

            WriteMode::Jsonl => {
                for data in vec {
                    writer.write_all("{".as_bytes())?;
                    writer.write_all(
                        json::to_string(&data.get_data_as_string(detail_level))
                            .as_bytes(),
                    )?;
                    writer.write_all("}\n".as_bytes())?;
                    // if i<vec.len()-1{
                    // writer.write_all(",\n".as_bytes())?;
                    // }
                }
                // writer.seek_relative(-1)
            }
        }
        // writer.write_all(serde_json::to_string(&self.to_vec()).expect("this failed").as_bytes()).unwrap();

        // BufWriter::new(file).write_all(pokemon);
        Ok(())
    }
}
impl From<SingleSearchReturn> for PokedexSearchResualt {
    fn from(value: SingleSearchReturn) -> Self {
        Self::Single(value)
    }
}
impl From<MultiSearchReturn> for PokedexSearchResualt {
    fn from(mut value: MultiSearchReturn) -> Self {
        value.sort_by(|o, t| {
            if o.get_dex_number() > t.get_dex_number() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        Self::Multi(value)
    }
}

pub const POKEDEX_DATA:&[u8;236014] = include_bytes!("../pokedex.jsonl");


pub struct PokeDex {
    mmap: Mmap,
}
impl PokeDex {
    pub fn new() -> Result<Self, std::io::Error> {
        // File::(include_str!("../pokedex.jsonl"));
        // let file = File::open("pokedex.jsonl").unwrap();
        // let data = ;
        let mut mmap = memmap2::MmapOptions::new().len(POKEDEX_DATA.len()).map_anon()?;
        mmap.copy_from_slice(POKEDEX_DATA);
        let mmap = mmap.make_read_only()?;
        Ok(Self { mmap })
    }
    #[allow(clippy::type_complexity)]
    fn mmap_to_pokemap(
        &self,
    ) -> rayon::iter::Map<
        rayon::iter::IterBridge<
            std::iter::MapWhile<
                std::io::Lines<&[u8]>,
                impl FnMut(Result<String, std::io::Error>) -> Option<String>,
            >,
        >,
        impl Fn(String) -> Pokemon,
    > {
        self.mmap
            .lines()
            .map_while(|item| item.ok())
            .par_bridge()
            .map(|line| json::from_str::<Pokemon>(&line).unwrap())
    }

    fn find_one_pokemon<P: Fn(&Pokemon) -> bool + Sync + Send>(
        &self,
        find: P,
    ) -> SingleSearchReturn {
        self.mmap_to_pokemap().find_first(find)
    }

    fn find_many_pokemon<P: Fn(&Pokemon) -> bool + Sync + Send>(
        &self,
        filter: P,
    ) -> MultiSearchReturn {
        self.mmap_to_pokemap().filter(filter).collect()
    }
    pub fn find_by_type(&self, ptype: PokemonType) -> MultiSearchReturn {
        // println!("find_by_type");
        self.find_many_pokemon(|pokemon| {
            pokemon.get_primary_type() == ptype || pokemon.get_seconary_type() == Some(ptype)
        })
    }

    pub fn find_by_natinal_dex_number(&self, dex_num: u16) -> SingleSearchReturn {
        self.find_one_pokemon(|pokemon| pokemon.get_dex_number() == dex_num)
    }
    pub fn find_by_name(&self, name: &String) -> SingleSearchReturn {
        self.find_one_pokemon(|pkmn| pkmn.get_name() == name)
    }
    pub fn find_by_color(&self, color: PokedexColor) -> MultiSearchReturn {
        self.find_many_pokemon(|pkmn| pkmn.get_color() == color)
    }
    pub fn find_by_stat(&self, stat: &StatWithOrder) -> MultiSearchReturn {
        self.find_many_pokemon(|pokemon| pokemon.stat_matches(stat))
    }
}

pub const MAX_POKEDEX_NUM: u16 = 1025;
