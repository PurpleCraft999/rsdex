use std::{cmp::Ordering, ops::Range, str::FromStr};

use crate::{compute_similarity, pokemon::Null};
use serde::{Deserialize, Serialize};
use strum::{Display,EnumString, VariantArray};

#[derive(
    Deserialize,
    PartialEq,
    Clone,
    Copy,
    EnumString,
    Display,
    VariantArray,
    Serialize,
    Eq,
    Hash,
    Debug,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Grass,
    Flying,
    Fighting,
    Poison,
    Electric,
    Ground,
    Rock,
    Psychic,
    Ice,
    Bug,
    Ghost,
    Steel,
    Dragon,
    Dark,
    Fairy,
    None,
}
impl<'n> Null<'n> for PokemonType {
    fn null() -> Self {
        Self::None
    }
}
#[derive(
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    EnumString,
    Display,
    VariantArray,
    Serialize,
    Eq,
    Hash,
    Debug,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum PokedexColor {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    Brown,
    Purple,
    Gray,
    White,
    Pink,
}

#[derive(Clone, Debug)]
pub struct StatWithOrder {
    pub stat: PokemonStat,
    pub operation: Ordering,
}

impl FromStr for StatWithOrder {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("parsing {s}");
        let stat = PokemonStat::from_str(s)?;
        let operation = if s.starts_with('l') {
            Ordering::Less
        } else if s.starts_with('g') {
            Ordering::Greater
        } else {
            Ordering::Equal
        };

        Ok(Self { stat, operation })
    }
}
pub fn stat_matches_ordering(order: Ordering, stat1: u8, stat2: u8) -> bool {
    match order {
        Ordering::Equal => stat1.cmp(&stat2).is_eq(),
        Ordering::Greater => stat1.cmp(&stat2).is_ge(),
        Ordering::Less => stat1.cmp(&stat2).is_le(),
    }
}
#[derive(Clone, Display, Debug)]
pub enum PokemonStat {
    Hp(u8),
    Attack(u8),
    Defence(u8),
    SpecialAttack(u8),
    SpecialDefence(u8),
    Speed(u8),
}

impl FromStr for PokemonStat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //it cant be a stat if it doesnt have a number or one of the letters
        if !s.contains([
            '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'd', 'h', 's',
        ]) {
            return Err("no number found".into());
        }

        match s {
            hp if s.ends_with("hp") => Ok(Self::Hp(str_to_u8(hp))),
            attack if s.ends_with('a') => Ok(Self::Attack(str_to_u8(attack))),
            defence if s.ends_with('d') => Ok(Self::Defence(str_to_u8(defence))),
            special_attack if s.ends_with("sa") => {
                Ok(Self::SpecialAttack(str_to_u8(special_attack)))
            }
            special_defence if s.ends_with("sd") => {
                Ok(Self::SpecialDefence(str_to_u8(special_defence)))
            }
            speed if s.ends_with('s') => Ok(Self::Speed(str_to_u8(speed))),
            _ => Err("could not parse stat from str".into()),
        }
    }
}
fn str_to_u8(s: &str) -> u8 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .expect("expected a number but none was found ")
}
#[derive(
    Deserialize, Clone, Serialize, Display, PartialEq, EnumString, VariantArray, Eq, Hash, Debug,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
///for whatever reason these names of some of them are different in the data set then else where
pub enum EggGroup {
    Monster,
    #[serde(alias = "humanshape")]
    HumanLike,
    Water1,
    Water2,
    Water3,
    #[strum(serialize = "bugegg")]
    #[strum(to_string = "bug")]
    Bug,
    Mineral,
    #[strum(serialize = "flyingegg")]
    #[strum(to_string = "flying")]
    Flying,
    #[serde(alias = "indeterminate")]
    #[strum(to_string = "amorphous")]
    Amorphous,
    #[serde(alias = "ground")]
    #[strum(to_string = "field")]
    Field,
    #[strum(serialize = "fairyegg")]
    #[strum(to_string = "fairy")]
    Fairy,
    Ditto,
    #[serde(alias = "plant")]
    #[strum(serialize = "grassegg")]
    #[strum(to_string = "grass")]
    Grass,
    #[strum(serialize = "dragonegg")]
    #[strum(to_string = "dragon")]
    Dragon,
    NoEggs,
    None,
}
impl<'d> Null<'d> for EggGroup {
    fn null() -> Self {
        Self::None
    }
}

#[derive(Serialize, Deserialize, EnumString, Clone, PartialEq, Eq, Hash, Debug, Display)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
#[serde(rename_all = "kebab-case")]
pub enum BodyShape {
    Quadruped,
    Upright,
    Armor,
    Squiggle,
    #[strum(serialize = "bug")]
    BugWings,
    Wings,
    Legs,
    Humanoid,
    Tentacles,
    Arms,
    Fish,
    Heads,
    Ball,
    Blob,
}
macro_rules! parse_if_ok {
    ($input:expr, $($parser:path => $query:ident);* $(;)?) => {
        $(
            if let Ok(val) = $parser($input){
                return Ok(SearchQuery::$query(val));
            }
        )*
    };
}
#[derive(Display, Clone, Debug)]
#[strum(serialize_all = "lowercase")]
// #[strum_discriminants(name(KeyWordToken),derive(EnumIs))]
pub enum KeyWord {
    And(Box<KeyWord>, Box<KeyWord>),
    Literal(SearchQuery),
    Or(Box<KeyWord>, Box<KeyWord>),
}
impl KeyWord {
    pub fn parse(tokens:&mut impl Iterator<Item = String>) -> Result<KeyWord,String> {
        // let mut tokens: std::vec::IntoIter<KeyWordToken> = parse_tokens(&raw).into_iter();
        // let mut keywords = Vec::new();

        let mut current_keyword=KeyWord::literal(&tokens.next().unwrap())?;


        while let Some(current_token) = tokens.next(){
            // let next_value = tokens.peek();
            if current_token=="and"{

                // let right_value = Self::parse(tokens)?;
                
                current_keyword = Self::and(current_keyword, Self::parse(tokens)?)?;
                // keywords.push(Self::and(raw[curent_token_position], right));
            }





        }
        Ok(current_keyword)

        // Vec::new()
        // let mut skip = false;
        // let mut processed_keywords = Vec::new();

        // for (mut i, token) in tokens.iter().enumerate() {
        //     // println!("i:{i}");
        //     // println!("token:{token:?}");
        //     // println!("value:{}",raw[i]);

        //     if skip {
        //         skip = false;
        //         continue;
        //     }
        //     // if i>0 && let Some(previous)= tokens.get(i-1){
        //     //     println!("{previous:?}");
        //     //     if previous==&KeyWordToken::And{
        //     //         continue;
        //     //     }
        //     // }

        //     // if token==&KeyWordToken::And{
        //     //     println!("THIS IS AND");
        //     // }

        //     // if let Some(next) = tokens.get(i + 1) {
        //     //     //dont proccess this token if it will be apart of the next
        //     //     if token_needs_previous(*next) {
        //     //         continue;
        //     //     }
        //     // }
        //     match token {
        //         KeyWordToken::And => {
        //             let keyword = KeyWord::and(&processed_keywords[i - 1], &raw[i + 1])
        //                 .expect("must be a search query before and after `and`");
        //             processed_keywords.remove(i - 1);
        //             raw.remove(i + 1);
        //             i-=1;
        //             // tokens.remove(i+1);
        //             processed_keywords.push(keyword);
        //             skip = true;
        //         }

        //         KeyWordToken::Literal => {
        //             processed_keywords
        //                 .push(KeyWord::literal(&raw[i]).expect("could not parse search query"));
        //         }

        //         // KeyWordToken::Or => (),
        //     }
        // }
        // // println!("{:?}",processed_keywords);
        // processed_keywords
    }


    

    pub fn get_just(&self) -> Option<SearchQuery> {
        match self {
            Self::Literal(q) => Some(q.clone()),
            _ => None,
        }
    }

    fn and(left: Self, right: Self) -> Result<KeyWord, String> {
        Ok(Self::And(
            Box::new(left),
            Box::new(right)),
        )
    }
    fn literal(name: &str) -> Result<KeyWord, String> {
        Ok(Self::Literal(SearchQuery::parser(name)?))
    }
    // fn from_token(token: KeyWordToken,value:&str){

    // }

    pub fn is_and(&self) -> Vec<SearchQuery> {
        let mut chain = Vec::new();
        match self {
            Self::And(one, two) => {
                match one.get_just() {
                    Some(some) => chain.push(some),
                    None => chain.append(&mut Self::is_and(one)),
                }
                match two.get_just() {
                    Some(some) => chain.push(some),
                    None => chain.append(&mut Self::is_and(one)),
                }
            }
            _ => (),
        }

        chain
    }
}
// fn parse_tokens(input: &Vec<String>) -> Vec<KeyWordToken> {
//     let mut tokens = Vec::new();
//     for token in input {
//         if KeyWord::is_keyword_and(&token) {
//             tokens.push(KeyWordToken::And);
//         } else {
//             tokens.push(KeyWordToken::Literal);
//         }
//     }
//     tokens
// }

// fn token_needs_previous(token: KeyWordToken) -> bool {
//     return matches!(token, KeyWordToken::And | KeyWordToken::Or);
// }

#[derive(Clone, Display, Debug)]
pub enum SearchQuery {
    NatDex(u16),
    Name(String),
    Type(PokemonType),
    Color(PokedexColor),
    Stat(StatWithOrder),
    EggGroup(EggGroup),
    Range(Range<u16>),
}
use crate::pokedex::POKEMON_NAME_ARRAY;
impl SearchQuery {
    pub fn parser(input: &str) -> Result<Self, String> {
        // if is_pokemon_name(input) {
        //     return Ok(Self::Name(input.into()));
        // }
        // let mut parsed: Option<SearchQuery> = None;
        parse_if_ok!(input,
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
impl From<SearchQuery> for KeyWord {
    fn from(value: SearchQuery) -> Self {
        KeyWord::Literal(value)
    }
}
