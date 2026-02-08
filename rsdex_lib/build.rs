// fn emit_warning<T: AsRef<str>>(message: T) {
//     println!("cargo:warning={}", message.as_ref());
// }
// mod hello;
fn main() {
    // emit_warning("test");
    // #[cfg(feature = "downloaded")]

    include_needed_data();

    #[cfg(feature = "downloaded")]
    on_pokedex_data_change();
}
pub const MAX_POKEDEX_NUM: u16 = 1025;
// #[cfg(feature = "downloaded")]

#[cfg(feature = "downloaded")]
use std::io::BufRead;
use std::{env, path::Path};

use serde::Deserialize;
///this only exist to not have to deseirialze the entire `Pokemon` struct when parsing
#[derive(Debug, Deserialize)]
pub struct PokemonName {
    pub name: String,
}

// #[cfg(feature = "downloaded")]

fn include_needed_data() {
    println!("cargo::rerun-if-changed=pokedex.jsonl");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let pokedex_data_path = Path::new(&out_dir).join("max_pokedex_num.rs");
    let max_pokemon_num = format!("pub const MAX_POKEDEX_NUM: u16 = {};", MAX_POKEDEX_NUM);
    std::fs::write(pokedex_data_path, max_pokemon_num).unwrap();

    // #[cfg(feature = "online")]
    // {

    // }
}

#[cfg(feature = "downloaded")]
fn on_pokedex_data_change() {
    // let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // let data_string = manifest_dir+"/pokedex.jsonl";
    println!("cargo::rerun-if-changed=pokedex.jsonl");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let pokedex_data_path = Path::new(&out_dir).join("pokedex_data.rs");

    let pokedex_data_const = include_bytes!("pokedex.jsonl");

    let mut vec = Vec::with_capacity(MAX_POKEDEX_NUM as usize);
    for line in pokedex_data_const.lines() {
        let line = line.expect("failed to read line");
        let name =
            serde_json::from_str::<PokemonName>(&line).expect("could not parse pokemon from line");
        vec.push(name.name);
    }
    let pokemon_arr: [String; MAX_POKEDEX_NUM as usize] =
        vec.try_into().expect("unable to turn vec into name_array");

    // let a = 1;

    let pokedex_data = format!(
        "pub const POKEDEX_DATA:&[u8;{}] = &{:?};",
        pokedex_data_const.len(),
        pokedex_data_const
    );
    let pokemon_name_arr = format!(
        "pub(crate) static POKEMON_NAME_ARRAY:[&str;{}] = {:?};",
        pokemon_arr.len(),
        pokemon_arr
    );

    std::fs::write(
        pokedex_data_path,
        String::from_iter([pokedex_data, pokemon_name_arr]),
    )
    .unwrap();
}
