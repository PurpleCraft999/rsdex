use std::{ str::FromStr};

use clap::Parser;

// use serde::Deserialize;

mod pokedex;
mod pokemon;
use pokedex::PokeDex;
use pokemon::{PokedexColor, PokemonType};
use strum::{Display, VariantArray};

use crate::{
    pokedex::{MAX_POKEDEX_NUM, PokedexSearchResualt},
    pokemon::{POKEMON_NAME_ARRAY,  StatWithOrder, compute_similarity},
};
fn main() {
    let args = Args::parse();

    let pokedex = match PokeDex::new() {
        Ok(dex) => dex,
        Err(e) => panic!("could not run: {e}"),
    };
    // let pkmn1 = pokedex.find_by_natinal_dex_number(5).unwrap();
    // let pkmn2 = pokedex.find_by_natinal_dex_number(7).unwrap();
    let pokemon: PokedexSearchResualt = match args.search_value {
        SearchValue::Dex { dex_num } => pokedex.find_by_natinal_dex_number(dex_num).into(),
        SearchValue::Name { name } => pokedex.find_by_name(&name).into(),
        SearchValue::Type { ptype } => pokedex.find_by_type(ptype).into(),
        SearchValue::Color { color } => pokedex.find_by_color(color).into(),
        SearchValue::Stat { stat } => pokedex.find_by_stat(&stat).into(),
    };
    pokemon.print_data(args.detailed);
}

///rsdex is a cli that allow you to locally search for pokemon like the pokedex would allow you to.
#[derive(clap::Parser)]
#[command(version)]
struct Args {
    ///takes a pokemon's name,color,type or dex number
    #[arg(value_parser = SearchValue::parser)]
    search_value: SearchValue,
    ///prints a more detailed version of the data
    #[arg(long, short)]
    detailed: bool,
}

#[derive(clap::Subcommand, Clone, Display)]
///test
enum SearchValue {
    Dex { dex_num: u16 },

    Name { name: String },
    Type { ptype: PokemonType },
    Color { color: PokedexColor },
    Stat { stat: StatWithOrder },
}
impl SearchValue {
    fn parser(input: &str) -> Result<Self, String> {
        if let Ok(dex_num) = input.parse::<u16>() {
            if (1..=MAX_POKEDEX_NUM).contains(&dex_num) {
                return Ok(SearchValue::Dex { dex_num });
            } else {
                return Err(format!(
                    "the search value must be between 1-{MAX_POKEDEX_NUM}"
                ));
            }
        } else if let Ok(ptype) = PokemonType::from_str(input) {
            // println!("is type");
            return Ok(Self::Type { ptype });
        } else if let Ok(color) = PokedexColor::from_str(input) {
            return Ok(SearchValue::Color { color });
        } else if let Ok(stat) = StatWithOrder::from_str(input) {
            

            // let stat = StatWithOrder { stat, operation };
            return Ok(SearchValue::Stat { stat });
        }
        for name in &POKEMON_NAME_ARRAY {
            if input == *name {
                return Ok(Self::Name { name: input.into() });
            }
        }

        let mut potintal_names = compute_similarity(input, &POKEMON_NAME_ARRAY);

        let mut err_vec = Vec::new();
        err_vec.append(&mut potintal_names);
        err_vec.append(&mut compute_similarity(input, PokedexColor::VARIANTS));
        err_vec.append(&mut compute_similarity(input, PokemonType::VARIANTS));
        let mut did_you_mean_str = String::with_capacity(err_vec.len());
        did_you_mean_str.push_str("did you mean:");
        for string in err_vec {
            did_you_mean_str.push_str(&string);
            did_you_mean_str.push(',');
        }
        did_you_mean_str.pop();
        Err(did_you_mean_str)
    }
}

#[test]
fn test_nat_dex_numbers() {
    let pokedex = PokeDex::new().unwrap();
    for dex_num in 1..=MAX_POKEDEX_NUM {
        let args = ["rsdex".into(), dex_num.to_string()];
        let args = Args::parse_from(args);
        match args.search_value {
            SearchValue::Dex { dex_num } => pokedex.find_by_natinal_dex_number(dex_num).unwrap(),
            e => panic!("nat dex test failed: number:{dex_num},value:{e}"),
        };
    }
}

#[test]
fn test_pokemon_names() {
    let pokedex = PokeDex::new().unwrap();
    for name in POKEMON_NAME_ARRAY {
        let args = ["rsdex".into(), name];
        let args = Args::parse_from(args);
        match args.search_value {
            SearchValue::Name { name } => pokedex.find_by_name(&name).unwrap(),
            e => panic!("name test failed: name:{name},value:{e}"),
        };
    }
}
