use crate::{
    data_types::{EggGroup, PokedexColor, PokemonType, SearchQuery, StatWithOrder},
    pokemon::Pokemon,
};
#[cfg(feature = "downloaded")]
use memmap2::Mmap;
#[cfg(feature = "downloaded")]
use std::io::BufRead;

#[cfg(feature = "downloaded")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use strum::{Display, EnumString};
// use serde::Deserialize;
use std::{
    // fs::File,
    collections::HashSet,
    ffi::OsStr,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
    str::FromStr,
};

pub type SingleSearchReturn = Option<Pokemon>;
pub type MultiSearchReturn = Vec<Pokemon>;
#[derive(Debug)]
pub struct PokedexSearchResualt {
    vec: Vec<Pokemon>,
}
impl PokedexSearchResualt {
    pub fn new(mut vec: Vec<Pokemon>) -> Self {
        vec.sort_by(|o, t| {
            if o.get_dex_number() > t.get_dex_number() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
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
            println!("sorry we couldnt find any thing matching our data");
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
        mut write_mode: Option<WriteMode>,
    ) -> io::Result<()> {
        println!("writing to {}", fp);
        let fp = Path::new(&fp);
        let file = File::create(fp)
            .unwrap_or_else(|e| panic!("sorry rsdex could not create your file because {e}"));

        let mut writer = BufWriter::new(file);

        //tries to determine write mode if not set
        if write_mode.is_none() {
            write_mode = match WriteMode::from_str(
                fp.extension()
                    .unwrap_or_else(|| OsStr::new("extension missing"))
                    .to_str()
                    .expect("sorry the file path isn't valid unicode"),
                // true,
            ) {
                Ok(w) => Some(w),
                Err(_) => {
                    return Err(std::io::Error::other("could not guess writemode "));
                }
            }
        }

        write_mode
            .expect("invailed write_mode state: still None")
            .write(&mut writer, &self.vec, detail_level)
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
    fn from(vec: MultiSearchReturn) -> Self {
        Self::new(vec)
    }
}
impl Default for PokedexSearchResualt {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
// const POKEDEX_DATA = include!()
#[derive(Clone, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum WriteMode {
    Json,
    Jsonl,
    // Guess,
    Csv,
}

impl WriteMode {
    fn write<W: Write>(
        &self,
        writer: &mut W,
        data: &[Pokemon],
        detail_level: u8,
    ) -> std::io::Result<()> {
        // let mut writer = BufWriter::new(file);

        match self {
            WriteMode::Json => {
                writer.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
            }

            WriteMode::Jsonl => {
                for pkmn in data {
                    writer.write_all(
                        (serde_json::to_string(&pkmn.get_as_map(detail_level))? + "\n").as_bytes(),
                    )?;
                }
            }
            // WriteMode::Guess => {
            //     return Err(std::io::Error::other(
            //         "could not set the write mode automaticly please set it manuely",
            //     ));
            // }
            WriteMode::Csv => {
                for (i, pkmn) in data.iter().enumerate() {
                    let mut string = String::new();
                    let vec = pkmn.get_as_vec(detail_level);
                    if i == 0 {
                        let mut head_string = String::new();
                        for (k, _) in &vec {
                            head_string.push_str(k);
                            head_string.push(',');
                        }
                        head_string.pop();
                        head_string.push('\n');
                        writer.write_all(head_string.as_bytes())?;
                    }

                    for (_, v) in vec {
                        string.push_str(&v);
                        string.push(',');
                    }
                    string.pop();
                    string.push('\n');
                    writer.write_all(string.as_bytes())?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(feature = "online")]

pub struct PokedexOnline {}
#[cfg(feature = "online")]

impl PokedexOnline {
    pub fn new() -> Self {
        use reqwest::blocking::Client;

        Self {}
    }
}

#[cfg(feature = "online")]
impl Pokedex for PokedexOnline {
    fn find_many_pokemon<P: Fn(&Pokemon) -> bool + Sync + Send>(
        &self,
        filter: P,
    ) -> MultiSearchReturn {
        Vec::new()
    }
    fn find_single_pokemon<P: Fn(&Pokemon) -> bool + Sync + Send>(
        &self,
        find: P,
    ) -> SingleSearchReturn {
        // let s =match  reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon/ditto"){
        //     Ok(u)=>u,
        //     Err(_) =>return None
        // };
        // // serde_json::from_str::<Pokemon>(s.text().unwrap())
        None
    }
}

include!(concat!(env!("OUT_DIR"), "/max_pokedex_num.rs"));

#[cfg(feature = "downloaded")]
include!(concat!(env!("OUT_DIR"), "/pokedex_data.rs"));

#[cfg(all(feature = "online", not(feature = "downloaded")))]
pub type PokedexStruct = PokedexOnline;

#[cfg(feature = "downloaded")]
pub type PokedexStruct = PokeDexMmap;

#[cfg(feature = "downloaded")]
pub struct PokeDexMmap {
    mmap: Mmap,
}
#[cfg(feature = "downloaded")]
impl PokeDexMmap {
    pub fn new_err() -> Result<Self, std::io::Error> {
        let mut mmap = memmap2::MmapOptions::new()
            .len(POKEDEX_DATA.len())
            .map_anon()?;
        mmap.copy_from_slice(POKEDEX_DATA);
        let mmap = mmap.make_read_only()?;
        Ok(Self { mmap })
    }
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::new_err().expect("could not build PokedexMmap")
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
}
#[cfg(feature = "downloaded")]
impl Pokedex for PokeDexMmap {
    fn find_single_pokemon<P: Fn(&Pokemon) -> bool + Sync + Send>(
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
}

pub trait Pokedex {
    fn find_many_pokemon<P: Fn(&Pokemon) -> bool + Sync + Send>(
        &self,
        filter: P,
    ) -> MultiSearchReturn;
    fn find_single_pokemon<P: Fn(&Pokemon) -> bool + Sync + Send>(
        &self,
        find: P,
    ) -> SingleSearchReturn;

    fn find_by_pokemon_type(&self, ptype: &PokemonType) -> MultiSearchReturn {
        self.find_many_pokemon(|pokemon| {
            pokemon.get_primary_type() == ptype || pokemon.get_seconary_type() == ptype
        })
    }

    fn find_by_natinal_dex_number(&self, dex_num: &u16) -> SingleSearchReturn {
        self.find_single_pokemon(|pokemon| pokemon.get_dex_number() == dex_num)
    }
    fn find_by_name(&self, name: &str) -> SingleSearchReturn {
        self.find_single_pokemon(|pkmn| pkmn.get_name() == name)
    }
    fn find_by_color(&self, color: &PokedexColor) -> MultiSearchReturn {
        self.find_many_pokemon(|pkmn| pkmn.get_color() == color)
    }
    fn find_by_stat(&self, stat: &StatWithOrder) -> MultiSearchReturn {
        self.find_many_pokemon(|pokemon| pokemon.stat_matches(stat))
    }
    fn find_by_egg_group(&self, group: &EggGroup) -> MultiSearchReturn {
        self.find_many_pokemon(|pokemon| {
            pokemon.get_egg_group_1() == group || pokemon.get_egg_group_2() == group
        })
    }
    fn search(&self, value: &SearchQuery) -> PokedexSearchResualt {
        match value {
            SearchQuery::NatDex { dex_num } => self.find_by_natinal_dex_number(dex_num).into(),
            SearchQuery::Name { name } => self.find_by_name(name).into(),
            SearchQuery::Type { ptype } => self.find_by_pokemon_type(ptype).into(),
            SearchQuery::Color { color } => self.find_by_color(color).into(),
            SearchQuery::Stat { stat } => self.find_by_stat(stat).into(),
            SearchQuery::EggGroup { group } => self.find_by_egg_group(group).into(),
        }
    }
    fn multi_search(&self, values: impl IntoIterator<Item = SearchQuery>) -> PokedexSearchResualt {
        let mut singles = Vec::new();

        let mut many = PokedexSearchResualt::default();
        for value in values {
            if value.finds_single() {
                if let Some(value)  = self.search(&value).get_if_single() {
                    singles.push(value.clone())
                }
            } else {
                many.merge(&mut self.search(&value));
            }
        }
        many = PokedexSearchResualt::new(many.find_dupes());
        many.vec.append(&mut singles);
        many
    }
}
