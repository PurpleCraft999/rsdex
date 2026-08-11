use std::ops::Range;

use strsim::damerau_levenshtein;

pub mod data_types;
pub mod pokedex;
pub mod pokemon;
pub mod search;

#[cfg(feature = "file_writing")]
pub use pokedex::WriteMode;
pub use {pokedex::MAX_POKEDEX_NUM, pokemon::Pokemon};

fn compute_similarity(string: &str, options: &[&str]) -> Vec<String> {
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

struct UselessError;

#[cfg(test)]
mod pokedex_tests {
    // use crate::{pokedex::Pokedex, pokemon::Pokemon};

    impl PokeDexMmap {
        fn get(&self, name: &str) -> Pokemon {
            self.find_by_name(&name.try_into().unwrap()).unwrap()
        }
        fn id(&self, id: u16) -> Pokemon {
            self.find_by_natinal_dex_number(&id.try_into().unwrap())
                .unwrap()
        }
    }

    pub(crate) type TestResult = Result<(), String>;

    use crate::{
        pokedex::{PokeDexMmap, Pokedex, PokedexSearchResult},
        pokemon::Pokemon,
        search::{KeyWord, SearchQuery, SearchQueryParsing},
    };

    struct PokemonD0 {
        nat_dex_num: u16,
        // name: &'a str,
    }
    impl PokemonD0 {
        fn matches(&self, find: &Pokemon) {
            // assert_eq!(&self.name, find.get_name());
            assert_eq!(find.get_dex_number(), &self.nat_dex_num);
        }
    }

    #[test]
    fn test_pokedex_on_bulbasaur() {
        let find_pokemon = PokemonD0 {
            // name: "bulbasaur",
            nat_dex_num: 1,
        };
        let dex = PokeDexMmap::new().unwrap();

        find_pokemon.matches(
            dex.search(&SearchQuery::nat_dex(1))
                .get_if_single()
                .unwrap(),
        );
        find_pokemon.matches(
            dex.search(&SearchQuery::Name("bulbasaur".try_into().unwrap()))
                .get_if_single()
                .unwrap(),
        );
    }
    #[test]
    fn multi_search_dual_type() -> TestResult {
        let dex = PokeDexMmap::new().unwrap();
        println!("{:?}", dex.mmap_to_pokemap().collect::<Vec<_>>());
        let result = dex.search_many(KeyWord::and(
            KeyWord::query(SearchQueryParsing::Type, "bug")?,
            KeyWord::query(SearchQueryParsing::Type, "flying")?,
        ));

        assert_eq!(
            result.to_vec()[0],
            PokedexSearchResult::new(vec![
                // dex.id(12)
                dex.get("BUTTERFREE"),
                // dex.get("SCYTHER"),
                // dex.get("LEDYBA"),
                // dex.get("LEDIAN"),
                // dex.get("YANMA"),
                // dex.get("BEAUTIFLY"),
                // dex.get("MASQUERAIN"),
                // dex.get("NINJASK"),
                // dex.get("MOTHIM"),
                // dex.get("COMBEE"),
                // dex.get("VESPIQUEN"),
                // dex.get("YANMEGA"),
                // dex.get("VIVILLON"),
            ])
            .to_vec()[0]
        );
        Ok(())
    }
    #[test]
    fn test_multi_search_one() -> TestResult {
        let dex = PokeDexMmap::new().unwrap();
        let result = dex.search_many(KeyWord::query(SearchQueryParsing::NatDex, "1")?);
        assert_eq!(result, PokedexSearchResult::new(vec![dex.get("bulbasaur")]));
        Ok(())
    }
    #[test]
    fn test_multi_search_two_differnt() -> TestResult {
        let dex = PokeDexMmap::new().unwrap();
        let result = dex.search_many(KeyWord::and(
            KeyWord::query(SearchQueryParsing::Type, "normal")?,
            KeyWord::query(SearchQueryParsing::EggGroup, "noeggs")?,
        ));
        assert_eq!(
            result,
            PokedexSearchResult::new(vec![
                dex.id(174),
                dex.id(298),
                dex.id(440),
                dex.id(446),
                dex.id(486),
                dex.id(493),
                dex.id(648),
                dex.id(772),
                dex.id(773),
                dex.id(1024)
            ])
        );
        Ok(())
    }
}

#[cfg(test)]
mod parsing {
    use crate::{
        data_types::{PokemonName, PokemonType},
        pokedex_tests::TestResult,
        search::{KeyWord, SearchQuery, SearchQueryParsing},
    };

    impl SearchQuery {
        fn parses_to(what_type: SearchQueryParsing, input: &str, other: Self) -> TestResult {
            Ok(assert_eq!(Self::parse(what_type, input)?, other))
        }
    }

    impl PokemonName {
        // const Charmander:Self = Self("Charmander".);
        fn charmander() -> Self {
            Self::new("Charmander")
        }
        fn type_null() -> Self {
            Self::new("Type-Null")
        }
    }

    #[test]
    fn test_keyword_parse_single_value() -> TestResult {
        let keyword = KeyWord::parse(&mut ["dex:1".to_owned()].into_iter())?;
        assert_eq!(KeyWord::query(SearchQueryParsing::NatDex, "1")?, keyword);
        Ok(())
    }
    #[test]
    fn test_and_parse() -> TestResult {
        let test = KeyWord::and(
            KeyWord::query(SearchQueryParsing::NatDex, "1")?,
            KeyWord::query(SearchQueryParsing::NatDex, "2")?,
        );
        assert_eq!(
            KeyWord::and(
                KeyWord::Query(SearchQuery::nat_dex(1)),
                KeyWord::Query(SearchQuery::nat_dex(2))
            ),
            test
        );
        Ok(())
    }
    #[test]
    fn test_or_parse() -> TestResult {
        let test = KeyWord::or(
            KeyWord::query(SearchQueryParsing::Type, "fire")?,
            KeyWord::query(SearchQueryParsing::Type, "water")?,
        );
        assert_eq!(
            KeyWord::or(
                KeyWord::Query(SearchQuery::Type(PokemonType::Fire)),
                KeyWord::Query(SearchQuery::Type(PokemonType::Water))
            ),
            test
        );
        Ok(())
    }
    #[test]
    fn test_nat_dex_parse() -> TestResult {
        SearchQuery::parses_to(SearchQueryParsing::NatDex, "539", SearchQuery::nat_dex(539))
    }
    #[test]
    fn test_pokemon_name_parse() -> TestResult {
        SearchQuery::parses_to(
            SearchQueryParsing::Name,
            "charmander",
            SearchQuery::Name(PokemonName::charmander()),
        )
    }
    #[test]
    fn test_pokemon_name_random_capitalization_parse() -> TestResult {
        SearchQuery::parses_to(
            SearchQueryParsing::Name,
            "cHarmAnDeR",
            SearchQuery::Name(PokemonName::charmander()),
        )
    }
    #[test]
    fn test_pokemon_name_with_dash_parse() -> TestResult {
        SearchQuery::parses_to(
            SearchQueryParsing::Name,
            "type-null",
            SearchQuery::Name(PokemonName::type_null()),
        )
    }
    #[test]
    fn test_range_parse() -> TestResult {
        SearchQuery::parses_to(SearchQueryParsing::Range, "1..4", SearchQuery::Range(0..5))
    }
    #[test]
    fn test_type_parse() -> TestResult {
        SearchQuery::parses_to(
            SearchQueryParsing::Type,
            "ground",
            SearchQuery::Type(PokemonType::Ground),
        )
    }
}
