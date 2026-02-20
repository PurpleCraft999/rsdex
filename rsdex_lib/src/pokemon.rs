use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

// use crate::pokedex::PokedexColor;
use crate::{
    UselessError,
    data_types::{
        BodyShape, EggGroup, PokedexColor, PokemonStat, PokemonType, StatWithOrder,
        stat_matches_ordering,
    },
};

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
    shape: BodyShape,
}
impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_display(0))
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

    pub fn get_as_vec(&self, detail_level: u8) -> Vec<(&str, String)> {
        let mut vec = Vec::new();
        vec.push(("name", capitalize_first_letter(&self.name)));
        vec.push(("national dex number", self.national_dex_number.to_string()));
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
            vec.push(("shape", self.shape.to_string()));
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

    pub fn print(&self, detail_level: u8) {
        println!("{}", self.get_display(detail_level));
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
pub fn is_pokemon_name(posible_name: &str) -> bool {
    for name in crate::pokedex::POKEMON_NAME_ARRAY {
        if posible_name == name {
            return true;
        }
    }
    false
}
pub(crate) fn is_pokemon_name_result(posible_name: &str) -> Result<String, UselessError> {
    if is_pokemon_name(posible_name) {
        Ok(posible_name.into())
    } else {
        Err(UselessError)
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
pub trait Null<'de>: Deserialize<'de> {
    fn null() -> Self;
}
