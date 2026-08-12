use std::{
    collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, sync::{LazyLock, Mutex},
};

use serde::{Deserialize, Serialize};
type Key = u64;
static ID_MAP: LazyLock<Mutex<HashMap<u64, String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
fn insert(value: String) -> Key {
    let mut g = DefaultHasher::new();
    value.hash(&mut g);
    let id = g.finish();
    ID_MAP.lock().map(|mut m| m.insert(id, value)).unwrap();
    return id;
}
fn get(id: &Key) -> Option<String> {
    ID_MAP.lock().map(|m| m.get(id).cloned()).unwrap()
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
fn from(value: &str) -> Key {
    let value = make_camel_case_from_kebab(value.to_lowercase());

    insert(value)
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






#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Debug)]
pub struct StringId(#[serde(deserialize_with = "str_to_id", serialize_with = "id_to_str")] Key);
impl StringId {
    pub fn new(value: &str) -> Self {
        Self(from(value))
    }
    pub fn value(&self) -> String {
        get(&self.0).expect("key was inserted when creating instance")
    }
}
