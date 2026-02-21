use std::ops::Range;

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

fn str_to_range(input: &str) -> Result<Range<u16>, UselessError> {
    //zero is not a valid input for this case
    if !input.contains("..") || !input.contains(['1', '2', '3', '4', '5', '6', '7', '8', '9']) {
        return Err(UselessError);
    }
    let (min, max) = input.split_at(input.find("..").unwrap());
    let min = min.parse::<u16>().unwrap();
    let max = max[2..].parse().unwrap();
    if min >= max || max > MAX_POKEDEX_NUM || min < 1 {
        return Err(UselessError);
    }
    Ok(min - 1..max + 1)
}
fn str_to_pokedex_num(input: &str) -> Result<u16, String> {
    if let Ok(dex_num) = input.parse::<u16>() {
        if (1..=MAX_POKEDEX_NUM).contains(&dex_num) {
            Ok(dex_num)
        } else {
            Err(format!(
                "the search value must be between 1-{MAX_POKEDEX_NUM}"
            ))
        }
    } else {
        Err("could not parse u16".into())
    }
}

struct UselessError;

// #[cfg(test)]
// mod tests {
//     // use crate::{pokedex::Pokedex, pokemon::Pokemon};

//     impl PokeDexMmap {
//         fn get(&self, name: &str) -> Pokemon {
//             self.find_by_name(name).unwrap()
//         }
//         fn id(&self, id: u16) -> Pokemon {
//             self.find_by_natinal_dex_number(&id).unwrap()
//         }
//     }

//     use crate::{
//         data_types::{EggGroup, PokemonType, SearchQuery},
//         pokedex::{PokeDexMmap, Pokedex, PokedexSearchResult},
//         pokemon::Pokemon,
//     };

//     struct PokemonD0<'a> {
//         nat_dex_num: u16,
//         name: &'a str,
//     }
//     impl PokemonD0<'_> {
//         fn matches(&self, find: &Pokemon) {
//             assert_eq!(&self.name, find.get_name());
//             assert_eq!(&self.nat_dex_num, find.get_dex_number());
//         }
//     }

//     #[test]
//     fn test_pokedex_on_bulbasaur() {
//         let find_pokemon = PokemonD0 {
//             name: "bulbasaur",
//             nat_dex_num: 1,
//         };
//         let dex = PokeDexMmap::new().unwrap();

//         find_pokemon.matches(
//             dex.search(&SearchQuery::NatDex(1))  .get_if_single()
//                                                 .unwrap(),
//         );
//         find_pokemon.matches(
//             dex.search(&SearchQuery::Name("bulbasaur".into()))
//                 .get_if_single()
//                 .unwrap(),
//         );
//     }
//     #[test]
//     fn multi_search_dual_type() {
//         let dex = PokeDexMmap::new().unwrap();
//         let result = dex.search_many([
//             SearchQuery::Type(PokemonType::Bug).into(),
//             SearchQuery::Type(PokemonType::Flying).into(),
//         ]);

//         assert_eq!(
//             result,
//             PokedexSearchResult::new(vec![
//                 dex.get("BUTTERFREE"),
//                 dex.get("SCYTHER"),
//                 dex.get("LEDYBA"),
//                 dex.get("LEDIAN"),
//                 dex.get("YANMA"),
//                 dex.get("BEAUTIFLY"),
//                 dex.get("MASQUERAIN"),
//                 dex.get("NINJASK"),
//                 dex.get("MOTHIM"),
//                 dex.get("COMBEE"),
//                 dex.get("VESPIQUEN"),
//                 dex.get("YANMEGA"),
//                 dex.get("VIVILLON")
//             ])
//         );
//     }
//     // #[test]
//     // fn test_multi_search_one() {
//     //     let dex = PokeDexMmap::new().unwrap();
//     //     let result = dex.search_many([SearchQuery::NatDex(1)]);
//     //     assert_eq!(result, PokedexSearchResult::new(vec![dex.get("bulbasaur")]))
//     // }
//     #[test]
//     fn test_multi_search_two_differnt() {
//         let dex = PokeDexMmap::new().unwrap();
//         let result = dex.search_many([
//             SearchQuery::Type(PokemonType::Normal).into(),
//             SearchQuery::EggGroup(EggGroup::NoEggs).into(),
//         ]);
//         assert_eq!(
//             result,
//             PokedexSearchResult::new(vec![
//                 dex.id(174),
//                 dex.id(298),
//                 dex.id(440),
//                 dex.id(446),
//                 dex.id(486),
//                 dex.id(493),
//                 dex.id(648),
//                 dex.id(772),
//                 dex.id(773),
//                 dex.id(1024)
//             ])
//         )
//     }
// }
