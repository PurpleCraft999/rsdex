use std::{ ops::Range};

use strsim::damerau_levenshtein;

use crate::pokedex::MAX_POKEDEX_NUM;

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

fn str_to_range(input:&str)->Result<Range<u16>, RangeParseError>{
    //zero is not a valid input for this case
    if !input.contains("..") || !input.contains(['1','2','3','4','5','6','7','8','9']){
        
        return Err(RangeParseError);
    }
    let (min,max) = input.split_at(input.find("..").unwrap());
    let min = min.parse::<u16>().unwrap();
    let max = max[2..].parse().unwrap();
    if min>=max || max>MAX_POKEDEX_NUM||min<1{
        return Err(RangeParseError);
    }
    Ok(min-1..max+1)

}

struct RangeParseError;

#[cfg(test)]
mod tests {
    // use crate::{pokedex::Pokedex, pokemon::Pokemon};

    use crate::{
        pokedex::{PokeDexMmap, Pokedex},
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
        let dex = PokeDexMmap::new().unwrap();

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
