use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
include!(concat!(env!("OUT_DIR"), "/pokemon_name.rs"));
// use crate::pokedex::PokedexColor;
use crate::data_types::{
    BodyShape, EggGroup, PokedexColor, PokemonStat, PokemonType, StatWithOrder,
    stat_matches_ordering,
};

#[derive(Deserialize, Clone, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct Pokemon {
    name: PokemonName,
    national_dex_number: u16,
    type1: PokemonType,
    #[serde(deserialize_with = "null_parser")]
    type2: PokemonType,

    color: PokedexColor,
    genus: String,

    ability1: String,
    #[serde(deserialize_with = "null_parser")]
    ability2: String,
    #[serde(deserialize_with = "null_parser")]
    hidden_ability: String,

    hp: u8,
    attack: u8,
    defence: u8,
    special_attack: u8,
    special_defence: u8,
    speed: u8,

    egg_group1: EggGroup,
    #[serde(deserialize_with = "null_parser")]
    egg_group2: EggGroup,
    shape: BodyShape,
}
impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_display(0))
    }
}
impl Pokemon {
    pub fn get_as_vec(&self, detail_level: u8) -> Vec<(&str, String)> {
        #[rustfmt::skip]
        let levels= [
            (0_u8, ("name", self.name.to_string())),
            (0, ("national dex number", self.national_dex_number.to_string())),
            (1, ("genus", self.genus.clone())),
            (1, ("primary type", self.type1.to_string())),
            (1, ("secondary type", self.type2.to_string())),
            (2, ("color", self.color.to_string())),
            (2, ("egg group 1", self.egg_group1.to_string())),
            (2, ("egg group 2", self.egg_group2.to_string())),
            (3,("ability",self.ability1.to_string())),
            (3,("ability",self.ability2.to_string())),
            (3,("hidden ability",self.hidden_ability.to_string())),
            (3, ("shape", self.shape.to_string())),
            (4, ("hp", self.hp.to_string())),
            (4, ("attack", self.attack.to_string())),
            (4, ("defence", self.defence.to_string())),
            (4, ("special attack", self.special_attack.to_string())),
            (4, ("special defence", self.special_defence.to_string())),
            (4, ("speed", self.speed.to_string())),
        ];

        let mut vec = Vec::new();
        for (level, data) in levels {
            if level <= detail_level {
                vec.push(data);
            }
        }

        vec
    }

    pub fn get_as_map(&self, detail_level: u8) -> HashMap<&str, String> {
        let vec = self.get_as_vec(detail_level);
        let mut map = HashMap::with_capacity(vec.len());
        for (k, v) in vec {
            map.insert(k, v);
        }
        map
    }

    pub fn get_display(&self, detail_level: u8) -> String {
        let mut data_string = String::new();
        for (k, v) in self.get_as_vec(detail_level) {
            if &v != "none" {
                data_string.push_str(&(k.to_owned() + ": " + &v + "\n"));
            }
            // data_string.push_str(&v);
        }
        // data_string.push('\n');

        data_string
    }

    // pub fn print(&self, detail_level: u8) {
    //     println!("{}", self.get_display(detail_level));
    //     // println!("print data")
    // }
    pub fn get_name(&self) -> &PokemonName {
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

fn null_parser<'de, D, N: Nullable<'de>>(deserializer: D) -> Result<N, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(N::null()))
}
///when string should be `none`
pub trait Nullable<'de>: Deserialize<'de> {
    fn null() -> Self;
}
// impl Display for PokemonName {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self:?}")
//     }
// }
// impl TryFrom<&str> for PokemonName {
//     type Error = String;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         PokemonName::from_str(value)
//     }
// }
