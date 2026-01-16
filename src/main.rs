use std::str::FromStr;

use clap::Parser;

// use serde::Deserialize;

mod pokedex;
mod pokemon;
use pokedex::PokeDex;
use pokemon::{PokedexColor, PokemonType};
use strum::VariantArray;

use crate::{
    pokedex::{MAX_POKEDEX_NUM, PokedexSearchResualt},
    pokemon::{POKEMON_NAME_ARRAY, compute_similarity},
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
        SearchValue::Name { name } => pokedex.find_by_name(name).into(),
        SearchValue::Type { ptype } => pokedex.find_by_type(ptype).into(),
        SearchValue::Color { color } => pokedex.find_by_color(color).into(),
    };
    pokemon.print_data(args.detailed);
}

///pokedex
#[derive(clap::Parser)]
#[command(version)]
struct Args {
    ///takes a pokemon's name,color,type or dex number
    #[arg(value_parser = SearchValue::parser)]
    search_value: SearchValue,
    ///prints a more detailed version of the data
    #[arg(long)]
    detailed: bool,
}

#[derive(clap::Subcommand, Clone)]
///test
enum SearchValue {
    
    Dex { dex_num: u16 },
    
    Name { name: String },
    Type {
        ptype: PokemonType,
    },
    Color {
        color: PokedexColor,
    },
}
impl SearchValue {
    fn parser(input: &str) -> Result<Self, String> {
        if let Ok(dex_num) = input.parse::<u16>() {
            if 1 <= dex_num && dex_num <= MAX_POKEDEX_NUM {
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
        }
        let mut potintal_name = compute_similarity(input, &POKEMON_NAME_ARRAY);

        if potintal_name.len() == 1 {
            let first_name = &potintal_name[0];
            if first_name == input {
                return Ok(SearchValue::Name {
                    name: first_name.clone(),
                });
            }
        }
        if !potintal_name.is_empty() {
            let mut err_vec = Vec::new();
            err_vec.append(&mut potintal_name);
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
        } else {
             Err("sorry we couldnt find anything".into())
        }
    }
}


#[test]
fn test_dex_numbers(){
    let pokedex =PokeDex::new().unwrap();
    for dex_num in 1..=MAX_POKEDEX_NUM{
        let args = ["rsdex".into(),dex_num.to_string()];
        let args = Args::parse_from(args);
        match args.search_value{
            SearchValue::Dex { dex_num }=>pokedex.find_by_natinal_dex_number(dex_num),
            _=>panic!("idk man:{dex_num}")
        };
    }
}