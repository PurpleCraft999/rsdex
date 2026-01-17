use crate::pokemon::{PokedexColor, Pokemon,  PokemonType, StatWithOrder};
use memmap2::Mmap;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    // fs::File,
    io::BufRead,
};

pub type SingleSearchReturn = Option<Pokemon>;
pub type MultiSearchReturn = Vec<Pokemon>;
pub enum PokedexSearchResualt {
    Single(SingleSearchReturn),
    Multi(MultiSearchReturn),
}
impl PokedexSearchResualt {
    pub fn print_data(&self, is_detailed: bool) {
        match self {
            PokedexSearchResualt::Single(single) => match single {
                Some(pkmn) => pkmn.print(is_detailed),
                None => println!("sorry we couldnt find any thing"),
            },
            PokedexSearchResualt::Multi(vec) => {
                for pkmn in vec {
                    pkmn.print(is_detailed)
                }
                if vec.is_empty() {
                    println!("sorry we couldnt find any thing")
                }
            }
        }
    }
    // pub fn get_similar_words(&self){
    //     ma
    // }
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

pub struct PokeDex {
    mmap: Mmap,
}
impl PokeDex {
    pub fn new() -> Result<Self, std::io::Error> {
        // File::(include_str!("../pokedex.jsonl"));
        // let file = File::open("pokedex.jsonl").unwrap();
        let data = include_bytes!("../pokedex.jsonl");
        let mut mmap = memmap2::MmapOptions::new().len(data.len()).map_anon()?;
        mmap.copy_from_slice(data);
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
    pub fn find_by_type(&self, ptype: PokemonType) -> MultiSearchReturn {
        // println!("find_by_type");
        self.find_many_pokemon(|pokemon| {
            pokemon.get_primary_type() == ptype || pokemon.get_seconary_type() == ptype
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
