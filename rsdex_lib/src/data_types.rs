use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

use crate::pokemon::Null;
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
    #[strum(serialize = "grasstype")]
    #[strum(to_string = "grass")]
    Grass,
    #[strum(serialize = "flyingtype")]
    #[strum(to_string = "flying")]
    Flying,
    Fighting,
    Poison,
    Electric,
    Ground,
    Rock,
    Psychic,
    Ice,
     #[strum(serialize = "bugtype")]
    #[strum(to_string = "bug")]
    Bug,
    Ghost,
    Steel,
    #[strum(serialize = "dragontype")]
    #[strum(to_string = "dragon")]
    Dragon,
    Dark,
    #[strum(serialize = "fairytype")]
    #[strum(to_string = "fairy")]
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

#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Display, Debug, PartialEq)]
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
        let stat_value = str_to_u8(s).map_err(|_| "could not parse stat".to_owned())?;

        match s {
            hp if s.ends_with("hp") => Ok(Self::Hp(stat_value)),
            attack if s.ends_with('a') => Ok(Self::Attack(stat_value)),
            defence if s.ends_with('d') => Ok(Self::Defence(stat_value)),
            special_attack if s.ends_with("sa") => Ok(Self::SpecialAttack(stat_value)),
            special_defence if s.ends_with("sd") => Ok(Self::SpecialDefence(stat_value)),
            speed if s.ends_with('s') => Ok(Self::Speed(stat_value)),
            _ => Err("could not parse stat from str".into()),
        }
    }
}
fn str_to_u8(s: &str) -> Result<u8, ParseIntError> {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
    // .expect("expected a number but none was found ")
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
