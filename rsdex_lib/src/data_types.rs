use std::{cmp::Ordering, fmt::Display, hash::Hash, num::ParseIntError, str::FromStr};

use crate::pokemon::Nullable;
use serde::Deserialize;
use strum::{Display, EnumString, VariantNames};
mod string_id {
    use std::{
        collections::HashMap,
        hash::{DefaultHasher, Hash, Hasher},
        sync::{LazyLock, Mutex},
    };

    use serde::{Deserialize, Serialize};
    static ID_MAP: LazyLock<Mutex<HashMap<u64, String>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    fn insert(id: u64, value: String) {
        ID_MAP.lock().map(|mut m| m.insert(id, value)).unwrap();
    }
    fn get(id: &u64) -> Option<String> {
        ID_MAP.lock().map(|m| m.get(id).cloned()).unwrap()
    }
    #[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Debug)]
    pub struct StringId(#[serde(deserialize_with = "str_to_id", serialize_with = "id_to_str")] u64);
    impl StringId {
        pub fn new(value: &str) -> Self {
            Self(from(value))
        }
        pub fn value(&self) -> String {
            get(&self.0).expect("key was inserted when creating instance")
        }
    }
    fn str_to_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opt = String::deserialize(deserializer)?;

        Ok(from(&opt))
    }
    fn id_to_str<D>(id: &u64, serializer: D) -> Result<D::Ok, D::Error>
    where
        D: serde::Serializer,
    {
        get(id).serialize(serializer)
    }
    fn from(value: &str) -> u64 {
        let value = make_camel_case_from_kebab(value.to_lowercase());
        let mut g = DefaultHasher::new();
        value.hash(&mut g);
        let id = g.finish();
        insert(id, value);
        id
    }
    fn make_camel_case_from_kebab(mut kebab: String) -> String {
        fn capitalize_first_letter(mut name: String) -> String {
            let first_letter = name.remove(0);
            name.insert(0, first_letter.to_ascii_uppercase());
            name
        }
        //replace the  `-`'s
        while let Some(dash_pos) = kebab.find("-") {
            kebab.remove(dash_pos);
            let lower = kebab.remove(dash_pos);
            kebab.insert(dash_pos, lower.to_ascii_uppercase());
        }
        capitalize_first_letter(kebab)
    }
}

use crate::data_types::string_id::StringId;
macro_rules! string_new_type {
    ($(#[$attributes:meta])*  $name:ident) => {
        #[cfg_attr(feature = "file_writing", derive(serde::Serialize))]
        #[derive(Clone, serde::Deserialize, PartialEq, Debug)]
        #[serde(from="StringId",into="StringId")]
        $(#[$attributes])*
        pub struct $name(StringId);
        impl $name {
            pub fn new(s: &str) -> Self {
                Self::from(StringId::new(s))
            }
        }
        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::new(s))
            }
        }
        impl TryFrom<&str> for $name {
            type Error = <Self as FromStr>::Err;
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                Self::from_str(s)
            }
        }
        impl TryFrom<String> for $name {
            type Error = <Self as FromStr>::Err;
            fn try_from(s: String) -> Result<Self, Self::Error> {
                Self::from_str(&s)
            }
        }
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.value())
            }
        }
        impl From<StringId> for $name{
            fn from(value:StringId)->Self{
                Self(value)
            }
        }
        // impl Into<StringId> for $name{
        //     fn into(self)->StringId{
        //         self.0
        //     }
        // }
        impl From<$name> for StringId{
            fn from(value:$name)->StringId{
                value.0
            }
        }
    };
}

string_new_type!(PokemonAbility);
// #[serde(deserialize_with = "name_parser")]
string_new_type!(PokemonName);
string_new_type!(PokemonGenus);

impl<'de> Nullable<'de> for PokemonAbility {
    fn null() -> Self {
        PokemonAbility::new("None")
    }
}
#[cfg_attr(feature = "file_writing", derive(serde::Serialize))]
#[derive(Deserialize, PartialEq, Clone, Copy, EnumString, Display, VariantNames, Debug)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    #[strum(serialize = "grasstype")]
    #[strum(to_string = "Grass")]
    Grass,
    #[strum(serialize = "flyingtype")]
    #[strum(to_string = "Flying")]
    Flying,
    Fighting,
    Poison,
    Electric,
    Ground,
    Rock,
    Psychic,
    Ice,
    #[strum(serialize = "bugtype")]
    #[strum(to_string = "Bug")]
    Bug,
    Ghost,
    Steel,
    #[strum(serialize = "dragontype")]
    #[strum(to_string = "Dragon")]
    Dragon,
    Dark,
    #[strum(serialize = "fairytype")]
    #[strum(to_string = "Fairy")]
    Fairy,
    None,
}
impl<'n> Nullable<'n> for PokemonType {
    fn null() -> Self {
        Self::None
    }
}
#[cfg_attr(feature = "file_writing", derive(serde::Serialize))]
#[derive(Deserialize, Clone, Copy, PartialEq, EnumString, Display, VariantNames, Debug)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
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
        let operation = match s {
            _greater if s.starts_with('g') => Ordering::Greater,
            _less if s.starts_with('l') => Ordering::Less,
            _ => Ordering::Equal,
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
            _hp if s.ends_with("hp") => Ok(Self::Hp(stat_value)),
            _attack if s.ends_with('a') => Ok(Self::Attack(stat_value)),
            _defence if s.ends_with('d') => Ok(Self::Defence(stat_value)),
            _special_attack if s.ends_with("sa") => Ok(Self::SpecialAttack(stat_value)),
            _special_defence if s.ends_with("sd") => Ok(Self::SpecialDefence(stat_value)),
            _speed if s.ends_with('s') => Ok(Self::Speed(stat_value)),
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
#[cfg_attr(feature = "file_writing", derive(serde::Serialize))]
#[derive(Deserialize, Clone, Display, PartialEq, EnumString, VariantNames, Debug)]
#[serde(rename_all = "kebab-case")]
#[strum(ascii_case_insensitive)]
///for whatever reason these names of some of them are different in the data set then else where
pub enum EggGroup {
    Monster,
    #[serde(alias = "humanshape")]
    HumanLike,
    Water1,
    Water2,
    Water3,
    #[strum(serialize = "bugegg")]
    #[strum(to_string = "Bug")]
    Bug,
    Mineral,
    #[strum(serialize = "flyingegg")]
    #[strum(to_string = "Flying")]
    Flying,
    #[serde(alias = "indeterminate")]
    #[strum(to_string = "Amorphous")]
    Amorphous,
    #[serde(alias = "ground")]
    #[strum(to_string = "Field")]
    Field,
    #[strum(serialize = "fairyegg")]
    #[strum(to_string = "Fairy")]
    Fairy,
    Ditto,
    #[serde(alias = "plant")]
    #[strum(serialize = "grassegg")]
    #[strum(to_string = "Grass")]
    Grass,
    #[strum(serialize = "dragonegg")]
    #[strum(to_string = "Dragon")]
    Dragon,
    NoEggs,
    None,
}
impl<'d> Nullable<'d> for EggGroup {
    fn null() -> Self {
        Self::None
    }
}
#[cfg_attr(feature = "file_writing", derive(serde::Serialize))]
#[derive(Deserialize, EnumString, Clone, PartialEq, Hash, Debug, Display)]
#[strum(ascii_case_insensitive)]
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
#[cfg_attr(feature = "file_writing", derive(serde::Serialize))]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Deserialize)]
pub struct NationalPokedexNumber(u16);
impl NationalPokedexNumber {
    pub fn new(dex_num: u16) -> Result<Self, InvalidDexNum> {
        if (1..=crate::max_pokedex_number()).contains(&dex_num) {
            Ok(Self(dex_num))
        } else {
            Err(InvalidDexNum)
        }
    }
    pub fn number(&self) -> u16 {
        self.0
    }
}
#[derive(Debug)]
pub struct InvalidDexNum;

impl FromStr for NationalPokedexNumber {
    type Err = InvalidDexNum;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(dex_num) = input.parse::<u16>() {
            Self::new(dex_num)
        } else {
            Err(InvalidDexNum)
        }
    }
}
impl PartialEq<u16> for NationalPokedexNumber {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}
impl Display for NationalPokedexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&u16> for NationalPokedexNumber {
    fn eq(&self, other: &&u16) -> bool {
        self.eq(*other)
    }
}

impl TryFrom<u16> for NationalPokedexNumber {
    type Error = InvalidDexNum;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
