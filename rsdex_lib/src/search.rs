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
        let current = tokens.next().unwrap();

        let mut current_keyword = if let Some(current) = current.split_once(['=', ':']) {
            let the_type = SearchQueryParsing::from_str(current.0).unwrap();
            let current_search = current.1;
            KeyWord::query(the_type, current_search)?
        } else if current.starts_with('#') {
            KeyWord::query(SearchQueryParsing::Type, &current[1..])?
        } else {
            return Err(format!("could not parse {}", current));
        };

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

    pub fn and(left: Self, right: Self) -> KeyWord {
        Self::And(Box::new(left), Box::new(right))
    }
    pub fn query(what_type: SearchQueryParsing, name: &str) -> Result<KeyWord, String> {
        Ok(Self::Query(SearchQuery::parse(what_type, name)?))
    }
    pub fn or(left: Self, right: Self) -> KeyWord {
        Self::Or(Box::new(left), Box::new(right))
    }
}
macro_rules! query_parser {
    ($the_type:expr,$input:expr, $($parser:path => $query:ident);* $(;)?) => {
        // match $input{
        $(
            // _ if let Ok(val) = $parser($input) => Ok(Self::$query(val)),
            if let Ok(val) = $parser($input){
                let s = Self::$query(val);

                if SearchQueryParsing::from(&s) == $the_type{
                    return Ok(s);
                }

            }
        )*
        // _=>Err(Self::parsing_error($input))
        // }
    };
}
#[derive(Clone, Display, Debug, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(name(SearchQueryParsing))]
// #[strum_discriminants(derive(strum::EnumString))]
pub enum SearchQuery {
    NatDex(NationalPokedexNumber),
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
    data_types::{
        EggGroup, NationalPokedexNumber, PokedexColor, PokemonAbility, PokemonName, PokemonType,
        StatWithOrder,
    },
};
impl SearchQuery {
    pub fn nat_dex(num: u16) -> Self {
        Self::NatDex(num.try_into().unwrap())
    }

    pub fn parse(what_type: SearchQueryParsing, input: &str) -> Result<Self, String> {
        query_parser!(what_type,input,
            PokemonName::from_str=>Name;
            NationalPokedexNumber::from_str=>NatDex;
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
        // err_vec.append(&mut compute_similarity(input, PokemonName::VARIANTS));
        err_vec.append(&mut compute_similarity(input, PokedexColor::VARIANTS));
        err_vec.append(&mut compute_similarity(input, PokemonType::VARIANTS));
        // err_vec.append(&mut compute_similarity(input, PokemonAbility::VARIANTS));
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
impl From<SearchQuery> for KeyWord {
    fn from(value: SearchQuery) -> Self {
        Self::Query(value)
    }
}
impl FromStr for SearchQueryParsing {
    type Err = UnknownSearchQueryKey;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SearchQueryParsing::*;
        match s {
            "num" | "number" | "nat_dex" | "#" | "dex" => Ok(NatDex),
            "name" => Ok(Name),
            "ability" | "a" => Ok(Ability),
            "type" | "t" => Ok(Type),
            "color" | "c" => Ok(Color),
            "stat" | "s" => Ok(Stat),
            "egg" | "egg_group" | "egg-group" | "egg group" => Ok(EggGroup),
            "range" | "in range" | "in_range" | "in-range" => Ok(Range),
            _ => Err(UnknownSearchQueryKey),
        }
    }
}

#[derive(Debug)]
pub struct UnknownSearchQueryKey;
