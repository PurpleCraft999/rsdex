use std::{path::PathBuf, str::FromStr as _};

use clap::{Parser, value_parser};
use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};
use rsdex_lib::{
    max_pokedex_number,
    pokedex::{PokeDexMmap, Pokedex},
    search::KeyWord,
    writing::WriteType,
};

fn main() {
    let args = RsdexArgs::parse();

    if let Some(other) = args.other {
        match other {
            OtherCommands::AmountOfPokemon => println!("{}", max_pokedex_number()),
        }

        return;
    }

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

    let search_queries =
        KeyWord::parse(&mut args.search_queries.into_iter()).expect("parsing failed");
    let mut search_result = pokedex.search_many(search_queries);

    if let Some(fp) = args.file_path {
        // let fp = Path::new(&fp);
        let file = std::fs::File::create(&fp)
            .unwrap_or_else(|e| panic!("sorry rsdex could not create your file because {e}"));

        let mut writer = std::io::BufWriter::new(file);
        let mut write_mode = args.write_mode;
        if write_mode.is_none() {
            write_mode = match WriteType::from_str(
                fp.extension()
                    .unwrap_or_else(|| std::ffi::OsStr::new("extension missing"))
                    .to_str()
                    .expect("sorry the file path isn't valid unicode"),
            ) {
                Ok(w) => Some(w),
                Err(_) => {
                    println!("{:?}", std::io::Error::other("could not guess writemode"));
                    return;
                }
            }
        }
        search_result
            .write_data(
                &mut writer,
                detail_level,
                write_mode.expect("invailed write_mode state: still None"),
                args.pretty,
            )
            .expect("something went wrong while saving your file");
        println!("writing successful")
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
    #[arg(long, aliases(["fp","filepath"]),short('p'))]
    file_path: Option<PathBuf>,
    #[arg(long, requires = "file_path",aliases(["mode"]))]
    write_mode: Option<WriteType>,
    #[arg(long, requires = "file_path")]
    pretty: bool,
    #[arg(long, short, exclusive(true))]
    help: bool,
    #[command(subcommand)]
    other: Option<OtherCommands>,
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
#[derive(clap::clap_derive::Subcommand, Clone)]
enum OtherCommands {
    #[command(name = "amount", alias = "amount_of_pokemon")]
    AmountOfPokemon,
}
// #[command(group(ArgGroup::new("others").args(["amount_of_pokemon","test"])))]
// struct OtherCommands{
//     #[arg(long,alias("amount"))]
//     amount_of_pokemon:bool,
//     #[arg(long)]
//     test:bool

// }
