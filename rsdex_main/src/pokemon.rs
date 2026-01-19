use std::{cmp::Ordering, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use strsim::damerau_levenshtein;
use strum::{Display, EnumString, VariantArray};

// use crate::pokedex::PokedexColor;

#[derive(Deserialize, PartialEq, Clone, Copy, EnumString, Display, VariantArray, Serialize)]
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
#[derive(Deserialize, Clone, Copy, PartialEq, EnumString, Display, VariantArray, Serialize)]
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

#[derive(Deserialize, Clone, Serialize)]
pub struct Pokemon {
    name: String,
    national_dex_number: u16,
    type1: PokemonType,
    #[serde(deserialize_with = "null_parser")]
    type2: PokemonType,

    color: PokedexColor,
    genus: String,

    hp: u8,
    attack: u8,
    defence: u8,
    special_attack: u8,
    special_defence: u8,
    speed: u8,

    egg_group1: EggGroup,
    #[serde(deserialize_with = "null_parser")]
    egg_group2: EggGroup,
}
impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_data_as_string(0))
    }
}
impl Pokemon {
    fn possible_empty_value_stringer<'n, N: Null<'n> + PartialEq + Display>(
        pos_null: &N,
    ) -> String {
        if *pos_null != N::null() {
            format!(" and {}", pos_null)
        } else {
            "".into()
        }
    }

    pub fn get_data_as_string(&self, detail_level: u8) -> String {
        let mut data_string = String::new();

        data_string
            .push_str(format!("No.{}    {}\n", self.national_dex_number, self.name).as_str());
        if detail_level >= 1 {
            data_string.push_str(format!("the {}\n", self.genus).as_str());
            //this section prints the types
            data_string.push_str(format!("{}", self.type1).as_str());
            data_string.push_str(Self::possible_empty_value_stringer(&self.type2).as_str());
            data_string.push_str(" type\n");
        }

        if detail_level >= 2 {
            data_string.push_str(format!("this pokemon is {}\n", self.color).as_str());
            data_string.push_str(format!("this pokemon is in the {}", self.egg_group1).as_str());
            data_string.push_str(Self::possible_empty_value_stringer(&self.egg_group2).as_str());
            data_string.push_str(" egg group(s)\n");
        }
        if detail_level >= 4 {
            data_string.push_str(format!("hp:{}\n", self.hp).as_str());
            data_string.push_str(format!("attack:{}\n", self.attack).as_str());
            data_string.push_str(format!("defence:{}\n", self.defence).as_str());
            data_string.push_str(format!("special attack:{}\n", self.special_attack).as_str());
            data_string.push_str(format!("special defence:{}\n", self.special_defence).as_str());
            data_string.push_str(format!("speed:{}\n", self.speed).as_str());
        }
        data_string
    }

    pub fn print(&self, detail_level: u8) {
        print!("{}", self.get_data_as_string(detail_level));
        // println!("print data")
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_dex_number(&self) -> u16 {
        self.national_dex_number
    }
    pub fn get_primary_type(&self) -> PokemonType {
        self.type1
    }
    pub fn get_seconary_type(&self) -> PokemonType {
        self.type2
    }
    pub fn get_color(&self) -> PokedexColor {
        self.color
    }
    pub fn get_egg_group_1(&self) -> &EggGroup {
        &self.egg_group1
    }
    pub fn get_egg_group_2(&self) -> &EggGroup {
        &self.egg_group2
    }
    pub fn stat_matches(&self, stat: &StatWithOrder) -> bool {
        let order = stat.operation;
        match stat.stat {
            PokemonStat::Hp(stat2) => stat_matches_ordering(order, self.hp, stat2),
            PokemonStat::Attack(stat2) => stat_matches_ordering(order, self.attack, stat2),
            PokemonStat::Defence(stat2) => stat_matches_ordering(order, self.defence, stat2),
            PokemonStat::SpecialAttack(stat2) => {
                stat_matches_ordering(order, self.special_attack, stat2)
            }
            PokemonStat::SpecialDefence(stat2) => {
                stat_matches_ordering(order, self.special_defence, stat2)
            }
            PokemonStat::Speed(stat2) => stat_matches_ordering(order, self.speed, stat2),
        }
    }
}

fn null_parser<'de, D, N: Null<'de>>(deserializer: D) -> Result<N, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(N::null()))
}
trait Null<'de>: Deserialize<'de> {
    fn null() -> Self;
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
fn stat_matches_ordering(order: Ordering, stat1: u8, stat2: u8) -> bool {
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
        if !s.contains(['1', '2', '3', '4', '5', '6', '7', '8', '9']) {
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
#[derive(Deserialize, Clone, Serialize, Display, PartialEq, EnumString, VariantArray)]
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
    Bug,
    Mineral,
    #[strum(serialize = "flyingegg")]
    Flying,
    #[serde(alias = "indeterminate")]
    Amorphous,
    #[serde(alias = "ground")]
    Field,
    #[strum(serialize = "fairyegg")]
    Fairy,
    Ditto,
    #[serde(alias = "plant")]
    #[strum(serialize = "grassegg")]
    Grass,
    #[strum(serialize = "dragonegg")]
    Dragon,
    NoEggs,
    Genderunknown,
    None,
}
impl<'d> Null<'d> for EggGroup {
    fn null() -> Self {
        Self::None
    }
}
