use std::path::PathBuf;

use clap::{Parser, value_parser};
use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};
use rsdex_lib::{
    pokedex::{PokeDexMmap, Pokedex, WriteMode},
    search::KeyWord,
};

fn main() {
    let args = RsdexArgs::parse();
    let detail_level = args.detailed;
    let pokedex = match PokeDexMmap::new() {
        Ok(dex) => dex,
        Err(e) => panic!("could not build pokedex because: {e}"),
    };

    if args.help {
        print_read_me();
        return;
    }

    if args.search_queries.is_empty() {
        println!("please add an argument or use --help for help");
        return;
    }

    let search_queries = KeyWord::parse(&mut args.search_queries.into_iter()).expect("paring failed");
    let mut search_result = pokedex.search_many(search_queries);

    if let Some(fp) = args.file_path {
        search_result
            .write_data_to_file(&fp, detail_level, args.write_mode, args.pretty)
            .expect("something went wrong while saving your file");
        println!("writing succesfull")
    } else {
        search_result.sort();
        search_result.print_data(detail_level);
    }
}

#[derive(clap::Parser)]
#[command(version, disable_help_flag = true)]
struct RsdexArgs {
    search_queries: Vec<String>,
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
include!(concat!(env!("OUT_DIR"), "/readme.rs"));
fn print_read_me() {
    let parser = pulldown_cmark::Parser::new(READ_ME);
    let mut list = false;
    for event in parser {
        match event {
            Event::SoftBreak => println!(),
            Event::HardBreak => println!(),
            Event::Code(code) => print!("\x1b[48;5;235m{code}\x1b[0m"),
            Event::Text(text) => {
                if list {
                    println!("* {text}")
                } else {
                    print!("{text}")
                }
            }
            //double new lines is intentional
            Event::Start(Tag::Heading { level, .. }) => match level {
                //bold
                HeadingLevel::H3 => print!("\n\n\x1B[1m"),
                //bold and underline
                HeadingLevel::H2 => print!("\n\n\x1B[1;4m"),
                _ => (),
            },
            Event::Start(Tag::List(..)) => list = true,
            Event::End(TagEnd::List(..)) => list = false,
            Event::End(TagEnd::Heading(_)) => println!("\x1b[0m\n"),
            _ => (),
        }
    }
    println!()
}
