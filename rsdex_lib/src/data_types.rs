use std::{cmp::Ordering, ops::Range, str::FromStr};

use crate::{compute_similarity, pokemon::Null};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};

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

#[derive(Clone)]
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
#[derive(Clone, Display)]
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

macro_rules! parse_query {
    ($input:expr, $($parser:path => $query:ident);* $(;)?) => {
        $(
            if let Ok(val) = $parser($input){
                return Ok(SearchQuery::$query(val));
            }
        )*
    };
}

#[derive(Clone, Display)]
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
        parse_query!(input,
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
        matches!(self, SearchQuery::Name { .. } | SearchQuery::NatDex { .. })
    }
}
