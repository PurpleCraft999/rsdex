use std::{io::Write, str::FromStr};

use clap::{Parser, ValueEnum, builder::PossibleValue, value_parser};

// use serde::Deserialize;
#[macro_use]
mod pokedex;
mod pokemon;
use pokedex::PokeDex;
use pokemon::{PokedexColor, PokemonType};
use strum::{Display, VariantArray};

use crate::{
    pokedex::MAX_POKEDEX_NUM,
    pokemon::{EggGroup, Pokemon, StatWithOrder, compute_similarity},
};
fn main() {
    let args = Args::parse();
    let detail_level = args.detailed;
    let pokedex = match PokeDex::new() {
        Ok(dex) => dex,
        Err(e) => panic!("could not build pokedex because: {e}"),
    };
    let pokemon = pokedex.search_many(args.search_queries);

    if let Some(fp) = args.file_path {
        pokemon
            .write_data_to_file(fp, detail_level, args.write_mode)
            .expect("something went wrong while saving your file");
        println!("writing succesfull")
    } else {
        pokemon.print_data(detail_level);
    }
}

///rsdex is a cli that allow you to locally search for pokemon like the pokedex would allow you to.
///
///a thanks to PokéApi for the data
#[derive(clap::Parser)]
#[command(version)]
struct Args {
    ///takes a pokemon's name,color,type,stat,egg group or dex number
    ///
    /// to get dex number a simple number will work
    ///
    /// for name entering any pokemon name will work
    ///
    /// for species with a space in their name use a `-`
    ///
    /// for color typing a pokemons color will work
    ///
    /// ***Stats***
    ///
    /// to search a stat you append the stat you want to a number at the end
    /// also you can put a `l` or `g` in front of the number to get pokemon with stats that are ≤ or ≥
    /// # Examples
    ///
    /// `rsdex 25hp` for pokemon with 25 hp exactly
    ///
    /// `rsdex 10sa` for pokemon with 10 special attack exactly
    ///
    /// `rsdex l120d` for pokemon with ≤ 120 defence
    ///
    /// `rsdex g110s` for pokemon with ≥ 110 speed
    ///
    #[arg(value_parser = SearchQuery::parser)]
    search_queries: Vec<SearchQuery>,
    // #[arg(value_parser = SearchValue::parser)]
    // /// if you have this you can search for pokemon meeting both criteria
    // /// some options are disabled such as name and dex number because it will always return an error
    // second_search_value: Option<SearchValue>,
    ///provides more detail the higher the number given
    #[arg(long, short,value_parser = value_parser!(u8).range(0..=5),default_value_t=0)]
    detailed: u8,
    ///will write to the given path with the specified data level in the format specified by write-mode
    #[arg(long,aliases(["fp"]))]
    file_path: Option<String>,
    #[arg(long, requires = "file_path")]
    write_mode: Option<WriteMode>,
}
#[derive(clap::Subcommand, Clone, Display)]
enum SearchQuery {
    NatDex { dex_num: u16 },
    Name { name: String },
    Type { ptype: PokemonType },
    Color { color: PokedexColor },
    Stat { stat: StatWithOrder },
    EggGroup { group: EggGroup },
}
use crate::pokedex::POKEMON_NAME_ARRAY;
impl SearchQuery {
    fn parser(input: &str) -> Result<Self, String> {
        // let pokemon_names = Po;
        for name in POKEMON_NAME_ARRAY {
            if input == name {
                return Ok(Self::Name { name: input.into() });
            }
        }
        if let Ok(dex_num) = input.parse::<u16>() {
            if (1..=MAX_POKEDEX_NUM).contains(&dex_num) {
                return Ok(SearchQuery::NatDex { dex_num });
            } else {
                return Err(format!(
                    "the search value must be between 1-{MAX_POKEDEX_NUM}"
                ));
            }
        } else if let Ok(ptype) = PokemonType::from_str(input) {
            return Ok(Self::Type { ptype });
        } else if let Ok(color) = PokedexColor::from_str(input) {
            return Ok(SearchQuery::Color { color });
        } else if let Ok(stat) = StatWithOrder::from_str(input) {
            return Ok(SearchQuery::Stat { stat });
        } else if let Ok(group) = EggGroup::from_str(input) {
            return Ok(SearchQuery::EggGroup { group });
        }
        Err(Self::parsing_error(input))
    }
    fn parsing_error(input: &str) -> String {
        let mut err_vec = Vec::new();
        err_vec.append(&mut compute_similarity(input, &POKEMON_NAME_ARRAY));
        err_vec.append(&mut compute_similarity(input, PokedexColor::VARIANTS));
        err_vec.append(&mut compute_similarity(input, PokemonType::VARIANTS));

        err_vec.append(&mut compute_similarity(input, EggGroup::VARIANTS));
        let mut did_you_mean_str = String::with_capacity(err_vec.len());
        if !err_vec.is_empty() {
            did_you_mean_str.push_str("did you mean: ");
            for string in err_vec {
                did_you_mean_str.push_str(&string);
                did_you_mean_str.push(',');
            }
            did_you_mean_str.pop();
            did_you_mean_str
        } else {
            "sorry we couldnt parse the info".into()
        }
    }
    // fn parser_restricted(input: &str) -> Result<SearchValue, String> {
    //     match Self::parser(input) {
    //         Ok(value) => match value {
    //             Self::Name { .. } => Err("cant have name for second arg".into()),
    //             Self::NatDex { .. } => Err("cant have dex num for second num".into()),
    //             ok => Ok(ok),
    //         },
    //         Err(e) => Err(e),
    //     }
    // }
    fn finds_single(&self) -> bool {
        matches!(self, SearchQuery::Name { .. } | SearchQuery::NatDex { .. })
    }
}
#[derive(Clone, Display)]
pub enum WriteMode {
    Json,
    Jsonl,
    // Guess,
    Csv,
}
impl ValueEnum for WriteMode {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Json => Some(PossibleValue::new("json").alias("Json")),
            Self::Jsonl => Some(PossibleValue::new("jsonl").alias("Jsonl")),
            // Self::Guess => Some(PossibleValue::new("Guess")),
            Self::Csv => Some(PossibleValue::new("csv")),
        }
    }
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Json, Self::Jsonl, Self::Csv]
    }
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

// #[test]
// fn test_nat_dex_numbers() {
//     let pokedex = PokeDex::new().unwrap();
//     for dex_num in 1..=MAX_POKEDEX_NUM {
//         let args = ["rsdex".into(), dex_num.to_string()];
//         let args = Args::parse_from(args);
//         match args.search_values {
//             SearchValue::NatDex { dex_num } => {
//                 pokedex.find_by_natinal_dex_number(&dex_num).unwrap()
//             }
//             e => panic!("nat dex test failed: number:{dex_num},value:{e}"),
//         };
//     }
// }

// #[test]
// fn test_pokemon_names() {
//     let pokedex = PokeDex::new().unwrap();
//     for name in POKEMON_NAME_ARRAY {
//         let args = ["rsdex", &name];
//         let args = Args::parse_from(args);
//         match args.search_values {
//             SearchValue::Name { name } => pokedex.find_by_name(&name).unwrap(),
//             e => panic!("name test failed: name:{},value:{e}", &name),
//         };
//     }
// }
#[test]
fn test_pokemon_stats() {
    let attack_args = ["rsdex".into(), "150a"];
    Args::parse_from(attack_args);
    let less_attack_args = ["rsdex".into(), "l150a"];
    Args::parse_from(less_attack_args);
    let special_defence_args = ["rsdex".into(), "120sd"];
    Args::parse_from(special_defence_args);
}
