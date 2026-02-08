use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

// use crate::pokedex::PokedexColor;
use crate::{
    data_types::{
        BodyShape, EggGroup, PokedexColor, PokemonStat, PokemonType, StatWithOrder,
        stat_matches_ordering,
    },
    pokedex::MAX_POKEDEX_NUM,
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

#[cfg(feature = "downloaded")]
use crate::pokedex::POKEMON_NAME_ARRAY;
pub fn get_name_array() -> [&'static str; MAX_POKEDEX_NUM as usize] {
    #[cfg(feature = "downloaded")]
    {
        return POKEMON_NAME_ARRAY;
    }
    //
    #[cfg(feature = "online")]
    {
        // use reqwest::blocking::Response;

        let response =minreq::get("https://raw.githubusercontent.com/PurpleCraft999/rsdex/refs/heads/master/pokemon_names.json").send().expect("could not send name array request");

        let vec = serde_json::from_slice::<Vec<String>>(response.as_bytes())
            .expect("response into vec failed");

        let vec: Vec<&'static mut str> = vec
            .into_iter()
            .map(|s| Box::leak(s.into_boxed_str()))
            .collect();
        let vec: Vec<&str> = vec.into_iter().map(|s| &*s).collect();
        TryInto::<[&str; MAX_POKEDEX_NUM as usize]>::try_into(vec)
            .expect("array length was not equal to MAX_POKEDEX_NUM")

        // serde_json::from_slice::<Vec<&str>>(response.text().expect("retiving name array took to long").as_bytes()).expect("could not parse array ").try_into().expect("array length was not equal to MAX_POKEDEX_NUM")
    }
}

#[cfg(feature = "online")]
impl crate::pokedex::PokedexOnline {
    pub fn rustemon_pokemon_to_rsdex_pokemon(
        &self,
        pokemon: rustemon::model::pokemon::Pokemon,
    ) -> Option<Pokemon> {
        use std::str::FromStr;

        use rustemon::Follow;
        let pokemon_info = self.block_on(pokemon.species.follow(&self.client));

        let type2 = pokemon.types.get(1);
        let type2 = match type2 {
            Some(t) => &t.type_.name,
            None => "null",
        };
        let egg2 = pokemon_info.egg_groups.get(1);
        let egg2 = match egg2 {
            Some(t) => &t.name,
            None => "null",
        };
        let mut genus = String::new();
        for i in pokemon_info.genera {
            if i.language.name == "en" {
                genus = i.genus;
                break;
            }
        }

        Some(Pokemon {
            name: pokemon.name,
            national_dex_number: pokemon.id as u16,
            type1: PokemonType::from_str(&pokemon.types[0].type_.name).ok()?,
            type2: PokemonType::from_str(type2).unwrap_or(PokemonType::None),
            color: PokedexColor::from_str(&pokemon_info.color.name).ok()?,
            hp: pokemon.stats[0].base_stat as u8,
            attack: pokemon.stats[1].base_stat as u8,
            defence: pokemon.stats[2].base_stat as u8,
            special_attack: pokemon.stats[3].base_stat as u8,
            special_defence: pokemon.stats[4].base_stat as u8,
            speed: pokemon.stats[5].base_stat as u8,
            egg_group1: EggGroup::from_str(&pokemon_info.egg_groups[0].name).ok()?,
            egg_group2: EggGroup::from_str(egg2).unwrap_or(EggGroup::None),
            shape: BodyShape::from_str(&pokemon_info.shape?.name).ok()?,
            genus,
        })
    }
}
