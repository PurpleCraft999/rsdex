use std::ops::Range;
use std::str::FromStr;
use strum::{Display, VariantArray};

#[derive(Display, Clone, Debug, PartialEq)]
#[strum(serialize_all = "lowercase")]
// #[strum_discriminants(name(KeyWordToken),derive(EnumIs))]
pub enum KeyWord {
    ///meets all requirements
    And(Box<KeyWord>, Box<KeyWord>),
    Literal(SearchQuery),
    Or(Box<KeyWord>, Box<KeyWord>),
}
impl KeyWord {
    pub fn parse(tokens: &mut impl Iterator<Item = String>) -> Result<KeyWord, String> {
        let mut current_keyword = KeyWord::literal(&tokens.next().unwrap())?;
        while let Some(current_token) = tokens.next() {
            current_keyword = match current_token.as_str() {
                "and" => Self::and(current_keyword, Self::parse(tokens)?),
                "or" => Self::or(current_keyword, Self::parse(tokens)?),
                other => return Err("can not reconize key word:".to_owned() + other),
            }
        }
        Ok(current_keyword)
    }

    pub fn and(left: Self, right: Self) -> KeyWord {
        Self::And(Box::new(left), Box::new(right))
    }
    pub fn literal(name: &str) -> Result<KeyWord, String> {
        Ok(Self::Literal(SearchQuery::parser(name)?))
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
    Name(String),
    Type(PokemonType),
    Color(PokedexColor),
    Stat(StatWithOrder),
    EggGroup(EggGroup),
    Range(Range<u16>),
}
use crate::{
    compute_similarity,
    data_types::{EggGroup, PokedexColor, PokemonType, StatWithOrder},
    pokedex::POKEMON_NAME_ARRAY,
};
impl SearchQuery {
    pub fn parser(input: &str) -> Result<Self, String> {
        // if is_pokemon_name(input) {
        //     return Ok(Self::Name(input.into()));
        // }
        // let mut parsed: Option<SearchQuery> = None;
        ok_parser!(input,
            crate::pokemon::is_pokemon_name_result=>Name;
            crate::str_to_pokedex_num=>NatDex;
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
        err_vec.append(&mut compute_similarity(input, &POKEMON_NAME_ARRAY));
        err_vec.append(&mut compute_similarity(input, PokedexColor::VARIANTS));
        err_vec.append(&mut compute_similarity(input, PokemonType::VARIANTS));

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
    pub fn finds_single(&self) -> bool {
        matches!(self, SearchQuery::Name(..) | SearchQuery::NatDex(..))
    }
    // pub fn can_be_applied_once(&self)->bool{
    //     matches!(self,Self::Color(..))
    // }
}
