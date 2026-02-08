#[cfg(all(feature = "downloaded",feature = "online"))]
compile_error!("can not have both 'downloaded' and 'online' features aviliable");

use strsim::damerau_levenshtein;

pub mod data_types;
pub mod pokedex;
pub mod pokemon;

pub fn compute_similarity<S: ToString>(string: &str, options: &[S]) -> Vec<String> {
    options
        .iter()
        .map(|s| {
            let s = s.to_string();
            (damerau_levenshtein(&s, string), s)
        })
        .filter(|(num, s)| *num < 3 && string != s)
        .map(|(_, s)| s)
        .collect()
}

// fn get_request(url:&str)->reqwest::blocking::Response{
//     let response = match reqwest::blocking::get(url){
//         Ok(r)=>r,
//         Err(e)=>panic!("could not get request because {e}")
//     };

//     response
// }

#[cfg(test)]
mod tests {
    // use crate::{pokedex::Pokedex, pokemon::Pokemon};

    use crate::{
        pokedex::{Pokedex, PokedexStruct},
        pokemon::Pokemon,
    };

    struct PokemonD0<'a> {
        nat_dex_num: u16,
        name: &'a str,
    }
    impl PokemonD0<'_> {
        fn matches(&self, find: Pokemon) {
            assert_eq!(&self.name, find.get_name());
            assert_eq!(&self.nat_dex_num, find.get_dex_number());
        }
    }

    #[test]
    fn test_pokedex_on_bulbasaur() {
        let find_pokemon = PokemonD0 {
            name: "bulbasaur",
            nat_dex_num: 1,
        };
        let dex = PokedexStruct::new();

        match dex.find_by_natinal_dex_number(&1) {
            Some(p) => find_pokemon.matches(p),
            None => panic!("pokedex somehow broke"),
        }
        match dex.find_by_name(find_pokemon.name) {
            Some(p) => find_pokemon.matches(p),
            None => panic!("it broke"),
        }
    }
}
