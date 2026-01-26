
use clap::{Parser, value_parser};
use rsdex::{data_types::SearchQuery, pokedex::{PokeDexMmap, Pokedex, WriteMode}};
fn main() {
    let args = Args::parse();
    let detail_level = args.detailed;
    let pokedex = match PokeDexMmap::new() {
        Ok(dex) => dex,
        Err(e) => panic!("could not build pokedex because: {e}"),
    };
    let pokemon = pokedex.multi_search(args.search_queries);

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
