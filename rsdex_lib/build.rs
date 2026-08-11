// fn emit_warning<T: AsRef<str>>(message: T) {
//     println!("cargo:warning={}", message.as_ref());
// }
fn main() {
    make_pokedex_data();
    make_pokemon_name_enum();
    make_pokemon_abilities_enum();
    make_pokemon_genus_enum();
}
const POKEMON_DATA: &[u8] = include_bytes!("pokedex.jsonl");

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
struct PokemonGenus {
    genus: String,
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

fn make_pokedex_data() {
    println!("cargo::rerun-if-changed=pokedex.jsonl");

    let pokedex_data = format!(
        "pub static POKEDEX_DATA:[u8;{}] = {:?};\n",
        POKEMON_DATA.len(),
        POKEMON_DATA
    );
    // let max_pokemon_num = format!("pub const MAX_POKEDEX_NUM: u16 = {};", MAX_POKEDEX_NUM);
    std::fs::write(out_dir("pokedex_data"), pokedex_data).unwrap();
}

fn make_pokemon_name_enum() {
    println!("cargo::rerun-if-changed=pokedex.jsonl");
    //the start of enum
    let mut name_enum = String::from(
        "#[cfg_attr(feature = \"file_writing\", derive(serde::Serialize))]#[derive(Clone,serde::Deserialize,PartialEq,Debug,strum::EnumString,strum::Display,strum::VariantNames)]#[strum(ascii_case_insensitive)]#[serde(rename_all = \"kebab-case\")]pub enum PokemonName{",
    );

    for line in POKEMON_DATA.lines() {
        let line = line.expect("failed to read line");
        let name = serde_json::from_str::<PokemonName>(&line)
            .expect("could not parse pokemon from line")
            .name;

        let original_name = capitalize_first_letter(name.clone());

        let name = make_camel_case_from_kebab(name);

        name_enum.push_str(&format!("#[strum(to_string = \"{original_name}\")]"));
        name_enum.push_str(&name);
        name_enum.push(',');
    }
    name_enum.push('}');
    std::fs::write(out_dir("pokemon_name"), name_enum).unwrap()
}

fn make_pokemon_abilities_enum() {
    println!("cargo::rerun-if-changed=pokedex.jsonl");
    fn add_ability(ability_enum: &mut String, ability: String) {
        let ability = capitalize_first_letter(ability);
        ability_enum.push_str(&format!("#[strum(to_string = \"{ability}\")]"));
        ability_enum.push_str(&(make_camel_case_from_kebab(ability) + ","));
    }
    let mut abliities = HashSet::new();
    let mut ability_enum = String::from(
        "#[cfg_attr(feature = \"file_writing\", derive(serde::Serialize))]#[derive(Clone,serde::Deserialize,PartialEq,Debug,strum::EnumString,strum::Display,strum::VariantNames)]#[strum(ascii_case_insensitive)]#[serde(rename_all = \"kebab-case\")]pub enum PokemonAbility{",
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

fn make_pokemon_genus_enum() {
    println!("cargo::rerun-if-changed=pokedex.jsonl");

    let mut genuses = HashSet::new();
    let mut genus_enum = String::from(
        "#[cfg_attr(feature = \"file_writing\", derive(serde::Serialize))]#[derive(Clone,serde::Deserialize,PartialEq,Debug,strum::EnumString,strum::Display,strum::VariantNames)]#[strum(ascii_case_insensitive)]pub enum PokemonGenus{",
    );
    for line in POKEMON_DATA.lines() {
        let line = line.expect("failed to read line");
        let genus = serde_json::from_str::<PokemonGenus>(&line)
            .expect("could not parse genus")
            .genus;

        if genuses.insert(genus.clone()) {
            genus_enum.push_str(&format!("#[serde(rename = \"{genus}\")]"));
            genus_enum.push_str(&format!("#[strum(to_string = \"{genus}\")]"));
            let enum_name = genus
                .replace(" ", "")
                .trim_end_matches("Pokémon")
                .to_string();
            genus_enum.push_str(&make_camel_case_from_kebab(enum_name));
            genus_enum.push(',');
        }
    }
    genus_enum.push('}');
    std::fs::write(out_dir("pokemon_genus"), genus_enum).unwrap()
}

fn make_camel_case_from_kebab(mut kebab: String) -> String {
    //replace the  `-`'s
    while let Some(dash_pos) = kebab.find("-") {
        kebab.remove(dash_pos);
        let lower = kebab.remove(dash_pos);
        kebab.insert(dash_pos, lower.to_ascii_uppercase());
    }
    capitalize_first_letter(kebab)
}

fn capitalize_first_letter(mut name: String) -> String {
    let first_letter = name.remove(0);
    name.insert(0, first_letter.to_ascii_uppercase());
    name
}
