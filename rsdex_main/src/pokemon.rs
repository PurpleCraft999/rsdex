use std::{cmp::Ordering, collections::HashMap, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use strsim::damerau_levenshtein;
use strum::{Display, EnumString, VariantArray};

// use crate::pokedex::PokedexColor;

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

#[derive(Deserialize, Clone, Serialize, PartialEq, Eq, Hash, Debug)]
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
    // fn possible_empty_value_stringer<'n, N: Null<'n> + PartialEq + Display>(
    //     pos_null: &N,
    // ) -> String {
    //     if *pos_null != N::null() {
    //         pos_null.to_string()
    //     } else {
    //         "".into()
    //     }
    // }

    pub fn get_data_as_vec(&self, detail_level: u8) -> Vec<(&str, String)> {
        let mut vec = Vec::new();
        vec.push(("national dex number", self.national_dex_number.to_string()));
        vec.push(("name", capitalize_first_letter(&self.name)));
        if detail_level >= 1 {
            vec.push(("genus", self.genus.clone()));
            //this section prints the types
            vec.push(("primary type", self.type1.to_string()));
            vec.push(("secondary type", self.type2.to_string()));
            // map.push((Self::possible_empty_value_stringer(&self.type2).as_str()));
            // map.push((" type\n"));
        }

        if detail_level >= 2 {
            vec.push(("color", self.color.to_string()));
            vec.push(("egg group 1", self.egg_group1.to_string()));
            vec.push(("egg group 2", self.egg_group2.to_string()));
            // map.push((" egg group(s)\n"));
        }
        if detail_level >= 4 {
            vec.push(("hp", self.hp.to_string()));
            vec.push(("attack", self.attack.to_string()));
            vec.push(("defence", self.defence.to_string()));
            vec.push(("special attack", self.special_attack.to_string()));
            vec.push(("special defence", self.special_defence.to_string()));
            vec.push(("speed", self.speed.to_string()));
        }
        vec
        // vec.iter()
        //     .map(|(k, v)| (k.to_string(), v.to_string()))
        //     .collect()
        // map
    }

    pub fn get_write_data(&self, detail_level: u8) -> HashMap<&str, String> {
        let vec = self.get_data_as_vec(detail_level);
        let mut map = HashMap::with_capacity(vec.len());
        for (k, v) in vec {
            map.insert(k, v);
        }
        map
    }

    pub fn get_data_as_string(&self, detail_level: u8) -> String {
        let mut data_string = String::new();
        for (k, v) in self.get_data_as_vec(detail_level) {
            data_string.push_str(&(k.to_owned() + ":" + &v + "\n"));

            // data_string.push_str(&v);
        }
        data_string.push('\n');

        data_string
    }

    pub fn print(&self, detail_level: u8) {
        print!("{}", self.get_data_as_string(detail_level));
        // println!("print data")
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_dex_number(&self) -> &u16 {
        &self.national_dex_number
    }
    pub fn get_primary_type(&self) -> &PokemonType {
        &self.type1
    }
    pub fn get_seconary_type(&self) -> &PokemonType {
        &self.type2
    }
    pub fn get_color(&self) -> &PokedexColor {
        &self.color
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

fn capitalize_first_letter(string: &str) -> String {
    if string.len() > 1 {
        let mut first_letter = string
            .chars()
            .next()
            .unwrap()
            .to_ascii_uppercase()
            .to_string();
        for (i, ch) in string.chars().enumerate() {
            if i != 0 {
                first_letter.push(ch);
            }
        }
        first_letter
    } else {
        string.to_uppercase()
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
    Genderunknown,
    None,
}
impl<'d> Null<'d> for EggGroup {
    fn null() -> Self {
        Self::None
    }
}
