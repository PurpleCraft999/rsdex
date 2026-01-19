// fn emit_warning<T: AsRef<str>>(message: T) {
//     println!("cargo:warning={}", message.as_ref());
// }
// mod hello;
fn main() {
    // emit_warning("test");
    on_pokedex_data_change();
}
pub const MAX_POKEDEX_NUM: u16 = 1025;

use std::{env, io::BufRead, path::Path};

use serde::Deserialize;
///this only exist to not have to deseirialze the entire `Pokemon` struct when parsing for the `make_pokemon_name_array!` macro
#[derive(Debug, Deserialize)]
pub struct PokemonName {
    pub name: String,
}

fn on_pokedex_data_change() {
    // let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // let data_string = manifest_dir+"/pokedex.jsonl";
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

    // println!("cargo::rerun-if-changed=pokedex.jsonl");
    println!("cargo::rerun-if-changed=build.rs");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let pokedex_data_path = Path::new(&out_dir).join("pokedex_data.rs");

    let pokedex_data = format!(
        "pub const POKEDEX_DATA:&[u8;{}] = &{:?};",
        pokedex_data_const.len(),
        pokedex_data_const
    );
    let max_pokemon_num = format!("pub const MAX_POKEDEX_NUM: u16 = {};", MAX_POKEDEX_NUM);
    let pokemon_name_arr = format!(
        "pub static POKEMON_NAME_ARRAY:[&str;{}] = {:?};",
        pokemon_arr.len(),
        pokemon_arr
    );
    std::fs::write(
        pokedex_data_path,
        String::from_iter([pokedex_data, max_pokemon_num, pokemon_name_arr]),
    )
    .unwrap();
}
