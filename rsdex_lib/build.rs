// fn emit_warning<T: AsRef<str>>(message: T) {
//     println!("cargo:warning={}", message.as_ref());
// }
fn main() {
    on_pokedex_data_change();
    make_pokemon_name_enum();
    make_pokemon_abilities_enum();
}
const POKEMON_DATA: &[u8] = include_bytes!("pokedex.jsonl");
const MAX_POKEDEX_NUM: u16 = 1025;

use std::{
    collections::HashSet,
    env,
    io::BufRead,
    path::{Path, PathBuf},
};

use serde::Deserialize;
///this only exist to not have to deseirialze the entire `Pokemon` struct
#[derive(Deserialize)]
struct PokemonName {
    name: String,
}
#[derive(Deserialize)]
struct PokemonAbility {
    ability1: Option<String>,
    ability2: Option<String>,
    hidden_ability: Option<String>,
}

///adds the .rs
fn out_dir(file_name: &str) -> PathBuf {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    Path::new(&out_dir).join(file_name.to_string() + ".rs")
}

fn on_pokedex_data_change() {
    println!("cargo::rerun-if-changed=pokedex.jsonl");

    let pokedex_data = format!(
        "pub static POKEDEX_DATA:[u8;{}] = {:?};",
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
"#[derive(Clone,serde::Deserialize,serde::Serialize,PartialEq,Eq,Hash,Debug,strum::EnumString,strum::Display,strum::VariantArray)]
#[strum(serialize_all = \"lowercase\", ascii_case_insensitive)]
#[serde(rename_all = \"kebab-case\")]
pub enum PokemonName{",
    );

    for line in POKEMON_DATA.lines() {
        let line = line.expect("failed to read line");
        let name =
            serde_json::from_str::<PokemonName>(&line).expect("could not parse pokemon from line");
        let name = name.name;

        let original_name = name.clone();

        let name = make_upper_camel_case_from_kebab(name);

        name_enum.push_str(&format!("#[strum(to_string = \"{original_name}\")]"));
        name_enum.push_str(&name);
        name_enum.push(',');
    }
    name_enum.push('}');
    std::fs::write(out_dir("pokemon_name"), name_enum).unwrap()
}

fn make_pokemon_abilities_enum() {
    fn add_ability(ability_enum: &mut String, ability: String) {
        ability_enum.push_str(&format!("#[strum(to_string = \"{}\")]", ability.clone()));
        ability_enum.push_str(&(make_upper_camel_case_from_kebab(ability) + ","));
    }
    let mut abliities = HashSet::new();
    let mut ability_enum = String::from(
"#[derive(Clone,serde::Deserialize,serde::Serialize,PartialEq,Eq,Hash,Debug,strum::EnumString,strum::Display,strum::VariantArray)]
#[strum(serialize_all = \"lowercase\", ascii_case_insensitive)]
#[serde(rename_all = \"kebab-case\")]
pub enum PokemonAbility{",
    );
    for line in POKEMON_DATA.lines() {
        let line = line.expect("failed to read line");

        let ability = serde_json::from_str::<PokemonAbility>(&line)
            .expect("could not parse pokemon from line");
        if let Some(ability) = ability.ability1
            && abliities.insert(ability.clone())
        {
            add_ability(&mut ability_enum, ability);
        }
        if let Some(ability) = ability.ability2
            && abliities.insert(ability.clone())
        {
            add_ability(&mut ability_enum, ability);
        }
        if let Some(ability) = ability.hidden_ability
            && abliities.insert(ability.clone())
        {
            add_ability(&mut ability_enum, ability);
        }
    }
    ability_enum.push_str("None");
    ability_enum.push('}');

    std::fs::write(out_dir("pokemon_ability"), ability_enum).unwrap()
}

fn make_upper_camel_case_from_kebab(mut kebab: String) -> String {
    //replace the  `-`'s
    while let Some(dash_pos) = kebab.find("-") {
        kebab.remove(dash_pos);
        let lower = kebab.remove(dash_pos);
        kebab.insert(dash_pos, lower.to_ascii_uppercase());
    }
    //capilize first letter
    let first_letter = kebab.remove(0);
    kebab.insert(0, first_letter.to_ascii_uppercase());
    kebab
}
