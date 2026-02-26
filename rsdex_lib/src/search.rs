use std::ops::Range;
use std::str::FromStr;
use strum::{Display, VariantNames};

#[derive(Display, Clone, Debug, PartialEq)]
pub enum KeyWord {
    And(Box<KeyWord>, Box<KeyWord>),
    Query(SearchQuery),
    /// either or
    Or(Box<KeyWord>, Box<KeyWord>),
}
impl KeyWord {
    pub fn parse(tokens: &mut impl Iterator<Item = String>) -> Result<KeyWord, String> {
        let mut current_keyword = KeyWord::query(&tokens.next().unwrap())?;
        //to easily use tokens inside the loop
        while let Some(current_token) = tokens.next() {
            current_keyword = match current_token.as_str() {
                "and" | "/" => Self::and(current_keyword, Self::parse(tokens)?),
                "or" => Self::or(current_keyword, Self::parse(tokens)?),
                other => return Err("can not reconize key word: '".to_owned() + other + "'"),
            }
        }
        Ok(current_keyword)
    }
    ///also parses at the end
    pub fn preparsing(mut tokens:Vec<String>)->Result<KeyWord, String>{
        for token in tokens.iter_mut(){
            *token = token.to_lowercase();
        }
        Self::parse(&mut tokens.into_iter())
    } 

    pub fn and(left: Self, right: Self) -> KeyWord {
        Self::And(Box::new(left), Box::new(right))
    }
    pub fn query(name: &str) -> Result<KeyWord, String> {
        Ok(Self::Query(SearchQuery::parser(name)?))
    }
    pub fn or(left: Self, right: Self) -> KeyWord {
        Self::Or(Box::new(left), Box::new(right))
    }
}
macro_rules! ok_parser {
    ($input:expr, $($parser:path => $query:ident);* $(;)?) => {
        $(
            if let Ok(val) = $parser($input){
                return Ok(Self::$query(val));
            }
        )*
    };
}
#[derive(Clone, Display, Debug, PartialEq)]
pub enum SearchQuery {
    NatDex(u16),
    Name(PokemonName),
    Ability(PokemonAbility),
    Type(PokemonType),
    Color(PokedexColor),
    Stat(StatWithOrder),
    EggGroup(EggGroup),
    Range(Range<u16>),
}
use crate::{
    compute_similarity,
    data_types::{EggGroup, PokedexColor, PokemonType, StatWithOrder,PokemonAbility, PokemonName},
};
impl SearchQuery {
    pub fn parser(input: &str) -> Result<Self, String> {
        // println!("{input}");\
        ok_parser!(input,
            PokemonName::from_str=>Name;
            crate::str_to_pokedex_num=>NatDex;
            PokemonAbility::from_str=>Ability;
            PokemonType::from_str=>Type;
            PokedexColor::from_str=>Color;
            StatWithOrder::from_str=>Stat;
            EggGroup::from_str=>EggGroup;
            crate::str_to_range=>Range;
        );

        Err(Self::parsing_error(input))
    }
    fn parsing_error(input: &str) -> String {
        let mut err_vec = Vec::new();
        err_vec.append(&mut compute_similarity(input, PokemonName::VARIANTS));
        err_vec.append(&mut compute_similarity(input, PokedexColor::VARIANTS));
        err_vec.append(&mut compute_similarity(input, PokemonType::VARIANTS));
        err_vec.append(&mut compute_similarity(input, PokemonAbility::VARIANTS));
        err_vec.append(&mut compute_similarity(input, EggGroup::VARIANTS));
        let mut did_you_mean_str = String::with_capacity(err_vec.len());
        if !err_vec.is_empty() {
            did_you_mean_str.push_str("did you mean: ");
            for string in err_vec {
                did_you_mean_str.push_str(&string);
                did_you_mean_str.push(',');
            }
            did_you_mean_str.pop();
            did_you_mean_str
        } else {
            "sorry we couldnt parse the info".into()
        }
    }
}
