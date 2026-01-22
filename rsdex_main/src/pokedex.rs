use crate::{
    SearchValue, WriteMode,
    pokemon::{EggGroup, PokedexColor, Pokemon, PokemonType, StatWithOrder},
};
use clap::ValueEnum;
use memmap2::Mmap;
use rayon::iter::{ParallelBridge, ParallelIterator};
// use serde::Deserialize;
use std::{
    // fs::File,
    collections::HashSet,
    ffi::OsStr,
    fs::File,
    io::{self, BufRead, BufWriter},
    path::Path,
};

pub type SingleSearchReturn = Option<Pokemon>;
pub type MultiSearchReturn = Vec<Pokemon>;
#[derive(Debug)]
pub struct PokedexSearchResualt {
    vec: Vec<Pokemon>,
}
impl PokedexSearchResualt {
    pub fn new(vec: Vec<Pokemon>) -> Self {
        Self { vec }
    }
    pub fn merge(&mut self, other: &mut PokedexSearchResualt) {
        self.vec.append(&mut other.vec);
    }
    ///returns the dupes
    pub fn find_dupes(&mut self) -> Vec<Pokemon> {
        let mut set = HashSet::new();
        let mut return_vec = Vec::new();
        for pkmn in &self.vec {
            if !set.insert(pkmn) {
                return_vec.push(pkmn.clone());
            }
        }
        return_vec
    }

    pub fn print_data(&self, detail_level: u8) {
        // let vec = self.to_vec();
        if self.vec.is_empty() {
            println!("sorry we couldnt find any thing matching out data");
            return;
        }
        for pokemon in &self.vec {
            pokemon.print(detail_level);
        }
    }
    fn get_if_single(&self) -> Option<&Pokemon> {
        if self.vec.len() == 1 {
            Some(&self.vec[0])
        } else {
            None
        }
    }
    pub fn write_data_to_file(
        &self,
        fp: String,
        detail_level: u8,
        mut write_mode: WriteMode,
    ) -> io::Result<()> {
        println!("writing to {}", fp);
        let fp = Path::new(&fp);
        let file = File::create(fp)
            .unwrap_or_else(|e| panic!("sorry rsdex could not create your file because {e}"));

        // let pokemon:&[u8] = &self.to_vec().iter().map(|pkmn|pkmn.get_data_as_string(detail_level)).map(|s|s.as_bytes()).flatten().copied().collect::<Vec<u8>>();
        let mut writer = BufWriter::new(file);

        // let vec = self.to_vec();
        //tries to determine write mode if not set
        if let WriteMode::Guess = write_mode {
            write_mode = WriteMode::from_str(
                fp.extension()
                    .unwrap_or_else(|| OsStr::new("unkown"))
                    .to_str()
                    .expect("sorry the file path isnt valid unicode"),
                true,
            )
            .unwrap_or(write_mode)
        }

        write_mode.write(&mut writer, &self.vec, detail_level)
    }
}
impl From<SingleSearchReturn> for PokedexSearchResualt {
    fn from(value: SingleSearchReturn) -> Self {
        match value {
            Some(v) => Self::new(vec![v]),
            None => Self::default(),
        }
    }
}
impl From<MultiSearchReturn> for PokedexSearchResualt {
    fn from(mut vec: MultiSearchReturn) -> Self {
        vec.sort_by(|o, t| {
            if o.get_dex_number() > t.get_dex_number() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        Self::new(vec)
    }
}
impl Default for PokedexSearchResualt {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
// const POKEDEX_DATA = include!()

include!(concat!(env!("OUT_DIR"), "/pokedex_data.rs"));
pub struct PokeDex {
    mmap: Mmap,
}
impl PokeDex {
    pub fn new() -> Result<Self, std::io::Error> {
        // File::(include_str!("../pokedex.jsonl"));
        // let file = File::open("pokedex.jsonl").unwrap();
        // let data = include_bytes!("../pokedex.jsonl");
        let mut mmap = memmap2::MmapOptions::new()
            .len(POKEDEX_DATA.len())
            .map_anon()?;
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
            .map(|line| serde_json::from_str::<Pokemon>(&line).unwrap())
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
    pub fn find_by_type(&self, ptype: &PokemonType) -> MultiSearchReturn {
        // println!("find_by_type");
        self.find_many_pokemon(|pokemon| {
            pokemon.get_primary_type() == ptype || pokemon.get_seconary_type() == ptype
        })
    }

    pub fn find_by_natinal_dex_number(&self, dex_num: &u16) -> SingleSearchReturn {
        self.find_one_pokemon(|pokemon| pokemon.get_dex_number() == dex_num)
    }
    pub fn find_by_name(&self, name: &String) -> SingleSearchReturn {
        self.find_one_pokemon(|pkmn| pkmn.get_name() == name)
    }
    pub fn find_by_color(&self, color: &PokedexColor) -> MultiSearchReturn {
        self.find_many_pokemon(|pkmn| pkmn.get_color() == color)
    }
    pub fn find_by_stat(&self, stat: &StatWithOrder) -> MultiSearchReturn {
        self.find_many_pokemon(|pokemon| pokemon.stat_matches(stat))
    }
    pub fn find_by_egg_group(&self, group: &EggGroup) -> MultiSearchReturn {
        self.find_many_pokemon(|pokemon| {
            pokemon.get_egg_group_1() == group || pokemon.get_egg_group_2() == group
        })
    }

    pub fn search(&self, value: &SearchValue) -> PokedexSearchResualt {
        match value {
            SearchValue::NatDex { dex_num } => self.find_by_natinal_dex_number(dex_num).into(),
            SearchValue::Name { name } => self.find_by_name(name).into(),
            SearchValue::Type { ptype } => self.find_by_type(ptype).into(),
            SearchValue::Color { color } => self.find_by_color(color).into(),
            SearchValue::Stat { stat } => self.find_by_stat(stat).into(),
            SearchValue::EggGroup { group } => self.find_by_egg_group(group).into(),
        }
    }

    pub fn search_many(
        &self,
        values: impl IntoIterator<Item = SearchValue>,
    ) -> PokedexSearchResualt {
        let mut singles = Vec::new();

        let mut many = PokedexSearchResualt::default();
        for value in values {
            if value.finds_single() {
                match self.search(&value).get_if_single() {
                    Some(value) => singles.push(value.clone()),
                    None => panic!("i did something wrong  (search many)"),
                }
            } else {
                many.merge(&mut self.search(&value));
            }
        }
        many.find_dupes();
        many.vec.append(&mut singles);
        many
    }
}
