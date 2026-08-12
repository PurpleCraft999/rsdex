use crate::{
    data_types::{
        EggGroup, NationalPokedexNumber, PokedexColor, PokemonAbility, PokemonName, PokemonType,
        StatWithOrder,
    },
    pokemon::Pokemon,
    search::{KeyWord, SearchQuery},
};
use memmap2::Mmap;
// use rayon::iter::{ParallelBridge, ParallelIterator};
// use serde::Deserialize;
#[cfg(feature = "file_writing")]
use std::io::{self, Write};
use std::{
    // fs::File,
    collections::HashSet,

    io::BufRead,
    ops::Range,
};

pub type SingleSearchReturn = Option<Pokemon>;
pub type MultiSearchReturn = Vec<Pokemon>;
#[derive(Debug, PartialEq)]
pub struct PokedexSearchResult {
    vec: Vec<Pokemon>,
}
impl PokedexSearchResult {
    pub fn new(vec: Vec<Pokemon>) -> Self {
        Self { vec }
    }
    pub fn append(&mut self, other: &mut PokedexSearchResult) {
        self.vec.append(&mut other.vec);
    }
    ///returns the dupes
    pub fn return_duplicate(&mut self) -> Vec<Pokemon> {
        let mut set = HashSet::new();
        let mut return_vec = Vec::new();
        for pkmn in &self.vec {
            if !set.insert(pkmn.get_dex_number()) {
                return_vec.push(pkmn.clone());
            }
        }
        return_vec
    }
    //sorts in dex order
    pub fn sort(&mut self) {
        self.vec
            .sort_by(|o, t| o.get_dex_number().cmp(t.get_dex_number()));
    }

    pub fn print_data(&self, detail_level: u8) {
        // let vec = self.to_vec();
        if self.vec.is_empty() {
            println!("sorry we couldn't find any thing in our data");
            return;
        }
        let mut out = String::new();
        for pokemon in &self.vec {
            out += &pokemon.get_display(detail_level);
            out += "\n"
        }
        println!("{out}")
    }
    pub fn get_if_single(&self) -> Option<&Pokemon> {
        if self.vec.len() == 1 {
            Some(&self.vec[0])
        } else {
            None
        }
    }
    #[cfg(feature = "file_writing")]
    pub fn write_data<W: Write>(
        &self,
        writer: &mut W,
        detail_level: u8,
        write_mode: crate::WriteType,
        pretty: bool,
    ) -> io::Result<()> {
        // println!("writing to {}", file_path.display());
        // // let fp = Path::new(&fp);
        // let file = File::create(file_path)
        //     .unwrap_or_else(|e| panic!("sorry rsdex could not create your file because {e}"));

        // let mut writer = BufWriter::new(file);

        //tries to determine write mode if not set
        // if write_mode.is_none() {
        //     write_mode = match WriteMode::from_str(
        //         file_path
        //             .extension()
        //             .unwrap_or_else(|| OsStr::new("extension missing"))
        //             .to_str()
        //             .expect("sorry the file path isn't valid unicode"),
        //     ) {
        //         Ok(w) => Some(w),
        //         Err(_) => {
        //             return Err(std::io::Error::other("could not guess writemode"));
        //         }
        //     }
        // }

        write_mode
            // .expect("invailed write_mode state: still None")
            .write(writer, &self.vec, detail_level, pretty)
    }
    pub fn to_vec(self) -> Vec<Pokemon> {
        self.vec
    }
}
impl From<SingleSearchReturn> for PokedexSearchResult {
    fn from(value: SingleSearchReturn) -> Self {
        match value {
            Some(v) => Self::new(vec![v]),
            None => Self::default(),
        }
    }
}
impl From<MultiSearchReturn> for PokedexSearchResult {
    fn from(vec: MultiSearchReturn) -> Self {
        Self::new(vec)
    }
}
impl Default for PokedexSearchResult {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

// pub const MAX_POKEDEX_NUM: u16 = 1025;
pub fn get_pokedex_data() -> Vec<u8> {
    #[cfg(all(debug_assertions, not(test)))]
    {
        std::fs::read("rsdex_lib/pokedex.jsonl").expect("file should be there for debugging")
    }
    #[cfg(any(test, not(debug_assertions)))]
    {
        // println!("{:?}",std::env::current_dir());
        std::fs::read(std::env::current_dir().unwrap().join("pokedex.jsonl"))
            .expect("file should exist for testing")
    }
    // #[cfg(all(not(debug_assertions),not(test)))]
    // {

    // }
}
pub fn max_pokedex_number() -> u16 {
    String::from_utf8(get_pokedex_data())
        .expect("file is utf-8 encoded")
        .lines()
        .count() as u16
}

pub struct PokeDexMmap {
    mmap: Mmap,
}
impl PokeDexMmap {
    pub fn new() -> Result<Self, std::io::Error> {
        let data = get_pokedex_data();
        let mut mmap = memmap2::MmapOptions::new().len(data.len()).map_anon()?;
        mmap.copy_from_slice(&data);
        let mmap = mmap.make_read_only()?;
        Ok(Self { mmap })
    }
    fn mmap_to_pokemap(&self) -> impl Iterator<Item = Pokemon> {
        self.mmap
            .lines()
            .map_while(|item| item.ok())
            // .par_bridge()
            .map(|line| serde_json::from_str::<Pokemon>(&line).unwrap())
    }
}

impl Pokedex for PokeDexMmap {
    fn find_single_pokemon<P: Fn(&Pokemon) -> bool + Sync + Send>(
        &self,
        find: P,
    ) -> SingleSearchReturn {
        self.mmap_to_pokemap().find(find)
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

    fn find_by_natinal_dex_number(&self, dex_num: &NationalPokedexNumber) -> SingleSearchReturn {
        self.find_single_pokemon(|pokemon| pokemon.get_dex_number() == dex_num)
    }
    fn find_by_name(&self, name: &PokemonName) -> SingleSearchReturn {
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
    fn find_within_range_nat_dex(&self, range: &Range<u16>) -> MultiSearchReturn {
        self.find_many_pokemon(|pokemon| range.contains(&pokemon.get_dex_number().number()))
    }
    fn find_by_ability(&self, ability: &PokemonAbility) -> MultiSearchReturn {
        self.find_many_pokemon(|pokemon| {
            pokemon.get_ability_1() == ability
                || pokemon.get_ability_2() == ability
                || pokemon.get_hidden_ability() == ability
        })
    }
    fn search(&self, value: &SearchQuery) -> PokedexSearchResult {
        match value {
            SearchQuery::NatDex(dex_num) => self.find_by_natinal_dex_number(dex_num).into(),
            SearchQuery::Name(name) => self.find_by_name(name).into(),
            SearchQuery::Type(ptype) => self.find_by_pokemon_type(ptype).into(),
            SearchQuery::Color(color) => self.find_by_color(color).into(),
            SearchQuery::Stat(stat) => self.find_by_stat(stat).into(),
            SearchQuery::EggGroup(group) => self.find_by_egg_group(group).into(),
            SearchQuery::Range(range) => self.find_within_range_nat_dex(range).into(),
            SearchQuery::Ability(ability) => self.find_by_ability(ability).into(),
        }
    }

    fn search_many(&self, keyword: KeyWord) -> PokedexSearchResult {
        match keyword {
            KeyWord::And(left, right) => {
                let mut result = self.search_many(*left);
                result.append(&mut self.search_many(*right));
                PokedexSearchResult::new(result.return_duplicate())
            }
            KeyWord::Query(query) => self.search(&query),
            KeyWord::Or(left, right) => {
                let mut result = self.search_many(*left);
                result.append(&mut self.search_many(*right));
                result
            }
        }
    }
}
