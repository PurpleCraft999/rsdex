// fn emit_warning<T: AsRef<str>>(message: T) {
//     println!("cargo:warning={}", message.as_ref());
// }
// mod hello;
fn main() {
    // emit_warning("test");
    on_pokedex_data_change();
    make_pokemon_name_enum();
}
const POKEMON_DATA: &[u8] = include_bytes!("pokedex.jsonl");
const MAX_POKEDEX_NUM: u16 = 1025;

use std::{
    env,
    io::BufRead,
    path::{Path, PathBuf},
};

use serde::Deserialize;
///this only exist to not have to deseirialze the entire `Pokemon` struct
#[derive(Debug, Deserialize)]
pub struct PokemonName {
    pub name: String,
}
///adds the .rs
fn out_dir(file_name: &str) -> PathBuf {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    Path::new(&out_dir).join(file_name.to_string() + ".rs")
}

fn on_pokedex_data_change() {
    println!("cargo::rerun-if-changed=pokedex.jsonl");

    let pokedex_data = format!(
        "pub const POKEDEX_DATA:&[u8;{}] = &{:?};",
        POKEMON_DATA.len(),
        POKEMON_DATA
    );
    let max_pokemon_num = format!("pub const MAX_POKEDEX_NUM: u16 = {};", MAX_POKEDEX_NUM);
    std::fs::write(
        out_dir("pokedex_data"),
        String::from_iter([pokedex_data, max_pokemon_num]),
    )
    .unwrap();
}

fn make_pokemon_name_enum() {
    println!("cargo::rerun-if-changed=pokedex.jsonl");
        //the start of enum
    let mut name_enum = String::from(
        "
        #[derive(Clone,serde::Deserialize,serde::Serialize,PartialEq,Eq,Hash,Debug,strum::EnumString,strum::Display,strum::VariantArray,)]
#[strum(serialize_all = \"lowercase\", ascii_case_insensitive)]
#[serde(rename_all = \"kebab-case\")]
pub enum PokemonName{",
    );
    
    for line in POKEMON_DATA.lines() {
        let line = line.expect("failed to read line");
        let name =
            serde_json::from_str::<PokemonName>(&line).expect("could not parse pokemon from line");
        let mut name = name.name;


        let original_name = name.clone();

        //make them camelCase from kebab-case
        while let Some(dash_pos) = name.find("-") {
            name.remove(dash_pos);
            let lower = name.remove(dash_pos);
            name.insert(dash_pos, lower.to_ascii_uppercase());
        }
        //capilize first letter
        let first_letter = name.remove(0);
        name.insert(0, first_letter.to_ascii_uppercase());
        name_enum.push_str(&format!("#[strum(to_string = \"{original_name}\")]"));
        name_enum.push_str(&name);
        name_enum.push(',');
    }
    name_enum.push('}');
    std::fs::write(out_dir("pokemon_name"), name_enum).unwrap()
}
