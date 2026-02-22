use crate::{
    data_types::{EggGroup, KeyWord, PokedexColor, PokemonType, SearchQuery, StatWithOrder},
    pokemon::Pokemon,
};
use memmap2::Mmap;
// use rayon::iter::{ParallelBridge, ParallelIterator};
use strum::{Display, EnumString};
// use serde::Deserialize;
use std::{
    // fs::File,
    collections::HashSet,
    ffi::OsStr,
    fs::File,
    io::{self, BufRead, BufWriter, Write},
    ops::Range,
    path::PathBuf,
    str::FromStr,
};

pub type SingleSearchReturn = Option<Pokemon>;
pub type MultiSearchReturn = Vec<Pokemon>;
#[derive(Debug, PartialEq)]
pub struct PokedexSearchResult {
    vec: Vec<Pokemon>,
}
impl PokedexSearchResult {
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
    pub fn append(&mut self, other: &mut PokedexSearchResult) {
        self.vec.append(&mut other.vec);
    }
    ///returns the dupes
    pub fn return_duplicate(&mut self) -> Vec<Pokemon> {
        let mut set = HashSet::new();
        let mut return_vec = Vec::new();
        for pkmn in &self.vec {
            if !set.insert(pkmn) {
                return_vec.push(pkmn.clone());
            }
        }
        // println!("{:?}",return_vec);
        // if return_vec.is_empty() {
        //     self.vec.clone()
        // } else {
        //     return_vec
        // }
        return_vec
    }

    pub fn print_data(&self, detail_level: u8) {
        // let vec = self.to_vec();
        if self.vec.is_empty() {
            println!("sorry we couldn't find any thing in our data");
            return;
        }
        for pokemon in &self.vec {
            pokemon.print(detail_level);
        }
    }
    pub fn get_if_single(&self) -> Option<&Pokemon> {
        if self.vec.len() == 1 {
            Some(&self.vec[0])
        } else {
            None
        }
    }
    pub fn write_data_to_file(
        &self,
        file_path: &PathBuf,
        detail_level: u8,
        mut write_mode: Option<WriteMode>,
        pretty: bool,
    ) -> io::Result<()> {
        println!("writing to {}", file_path.display());
        // let fp = Path::new(&fp);
        let file = File::create(file_path)
            .unwrap_or_else(|e| panic!("sorry rsdex could not create your file because {e}"));

        let mut writer = BufWriter::new(file);

        //tries to determine write mode if not set
        if write_mode.is_none() {
            write_mode = match WriteMode::from_str(
                file_path
                    .extension()
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
            .write(&mut writer, &self.vec, detail_level, pretty)
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
        pretty: bool,
    ) -> io::Result<()> {
        if data.is_empty() {
            return std::io::Result::Err(io::Error::other("data cant be empty"));
        }

        match self {
            WriteMode::Json => {
                //makes it a json array
                writer.write_all("[".as_bytes())?;
                let mut json_string = String::new();
                for pkmn in data {
                    let pkmap = &pkmn.get_as_map(detail_level);
                    let pokemon_string = if pretty {
                        serde_json::to_string_pretty(pkmap)?
                    } else {
                        serde_json::to_string(pkmap)?
                    };
                    json_string += (pokemon_string + ",").as_str();
                }
                //removes the trailing comma
                json_string.pop();
                writer.write_all(json_string.as_bytes())?;
                writer.write_all("]".as_bytes())?;
            }
            //def no copied from json
            WriteMode::Jsonl => {
                let mut jsonl_string = String::new();
                for pkmn in data {
                    let pkmap = &pkmn.get_as_map(detail_level);
                    let pokemon_string = if pretty {
                        serde_json::to_string_pretty(pkmap)?
                    } else {
                        serde_json::to_string(pkmap)?
                    };
                    jsonl_string += (pokemon_string + "\n").as_str();
                }
                // no newline at end
                jsonl_string.pop();
                writer.write_all(jsonl_string.as_bytes())?;
            }
            WriteMode::Csv => {
                let mut csv_string = String::new();

                for (column_name, _) in &data[0].get_as_vec(detail_level) {
                    csv_string.push_str(column_name);
                    csv_string.push(',');
                }
                csv_string.push('\n');

                for pkmn in data {
                    let vec = pkmn.get_as_vec(detail_level);

                    for (_, column_value) in vec {
                        csv_string.push_str(&column_value);
                        csv_string.push(',');
                    }
                    csv_string.push('\n');
                }
                csv_string = csv_string.replace(",\n", "\n");
                writer.write_all(csv_string.as_bytes())?;
            }
        }

        Ok(())
    }
}

include!(concat!(env!("OUT_DIR"), "/pokedex_data.rs"));

pub struct PokeDexMmap {
    mmap: Mmap,
}
impl PokeDexMmap {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut mmap = memmap2::MmapOptions::new()
            .len(POKEDEX_DATA.len())
            .map_anon()?;
        mmap.copy_from_slice(POKEDEX_DATA);
        let mmap = mmap.make_read_only()?;
        Ok(Self { mmap })
    }
    #[allow(clippy::type_complexity)]
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

    fn find_by_natinal_dex_number(&self, dex_num: &u16) -> SingleSearchReturn {
        self.find_single_pokemon(|pokemon| pokemon.get_dex_number() == dex_num)
    }
    fn find_by_name(&self, name: &str) -> SingleSearchReturn {
        let name: &str = &name.to_lowercase();
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
        self.find_many_pokemon(|pokemon| range.contains(pokemon.get_dex_number()))
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
        }
    }

    // fn search()

    ///make this better
    fn search_many(&self, keyword:KeyWord) -> PokedexSearchResult {


        match keyword{
            KeyWord::And(left,right )=>{
                let mut result =self.search_many(*left);
                // let mut two = ;
                result.append(&mut self.search_many(*right));
                PokedexSearchResult::new( result.return_duplicate())
            },
            KeyWord::Literal(query)=>self.search(&query),
            KeyWord::Or(left,right)=>{
                let mut result =self.search_many(*left);
                result.append(&mut self.search_many(*right));
                result


            }





        }







        // let mut single = Vec::new();
        // let mut set = HashSet::new();
        // for query in queries{

        //     match query{
        //         search if search.finds_single()=>single.push(search),
        //         once if once.can_be_applied_once() => {single.push(once);},
        //         checkmate=>()
        //     }
        // }

        // let mut singles = Vec::new();
        // let mut many = PokedexSearchResult::default();
        // for query in queries {
        //     if query.finds_single() {
        //         if let Some(pokemon) = self.search_single(&query).get_if_single() {
        //             singles.push(pokemon.clone())
        //         }
        //     } else {
        //         many.merge(&mut self.search_single(&query));
        //     }
        // }
        // many = PokedexSearchResult::new(many.filter_for_search());
        // many.vec.append(&mut singles);
        // many
        // let mut results = PokedexSearchResult::default();
        
        //     match keyword {
        //         KeyWord::And(..) => {
        //             // println!("and ");
        //             let thing = keyword.is_and();
        //             for thing in thing{
        //                                         // let mut search = ;
        //             // search.append(&mut self.search(&thing[1]));
        //             // println!("{search:?}");
        //                 results.append(&mut self.search(&thing));


        //             }
        //             results =PokedexSearchResult::new(results.return_dupelicates());

                    
        //             // results = PokedexSearchResult::new(results.remove_dupelicates());

        //         }
        //         KeyWord::Literal(query) => results.append(&mut self.search(&query)),
        //         KeyWord::Or(..) => unreachable!(),
        //     }
        //     // match token{
        //     //     SearchToken::Query(query)=>results.merge(&mut self.search_single(&query)),
        //     //     SearchToken::KeyWord(keyword)=>()
        //     // }
        //     // results.merge(&mut self.search_single(&query))
        // }

        // results
        // PokedexSearchResult::default()
    }
}
