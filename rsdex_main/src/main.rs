use std::path::PathBuf;

use clap::{Parser, value_parser};
use rsdex_lib::{
    data_types::SearchQuery,
    pokedex::{PokeDexMmap, Pokedex, WriteMode},
};

include!(concat!(env!("OUT_DIR"), "/readme.rs"));

fn main() {
    let args = RsdexArgs::parse();
    let detail_level = args.detailed;
    let pokedex = match PokeDexMmap::new() {
        Ok(dex) => dex,
        Err(e) => panic!("could not build pokedex because: {e}"),
    };

    if args.help {
        termimad::print_text(READ_ME);

        return;
    }
    let search_queries = args.search_queries;

    if search_queries.is_empty() {
        println!("please add an argument or use --help for help");
        return;
    }

    let pokemon = if search_queries.len() == 1 {
        pokedex.search_single(&search_queries[0])
    } else {
        pokedex.search_many(search_queries)
    };

    // let pokemon = pokedex.search_many(args.search_queries);

    if let Some(fp) = args.file_path {
        pokemon
            .write_data_to_file(&fp, detail_level, args.write_mode, args.pretty)
            .expect("something went wrong while saving your file");
        println!("writing succesfull")
    } else {
        pokemon.print_data(detail_level);
    }
}

#[derive(clap::Parser)]
#[command(version, disable_help_flag = true)]
struct RsdexArgs {
    #[arg(value_parser = SearchQuery::parser)]
    search_queries: Vec<SearchQuery>,

    #[arg(long, short,value_parser = value_parser!(u8).range(0..=5),default_value_t=0)]
    detailed: u8,
    #[arg(long, alias("fp"))]
    file_path: Option<PathBuf>,
    #[arg(long, requires = "file_path")]
    write_mode: Option<WriteMode>,
    #[arg(long, requires = "file_path")]
    pretty: bool,
    #[arg(long)]
    help: bool,
}
