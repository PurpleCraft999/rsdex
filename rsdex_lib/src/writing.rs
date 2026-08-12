#[cfg(feature = "file_writing")]
use std::io::{self, Write};

use strum::{Display, EnumString};

#[cfg(feature = "file_writing")]
use crate::Pokemon;

#[derive(Clone, Display, EnumString, Default)]
#[strum(ascii_case_insensitive)]
pub enum WriteType {
    Json,
    Jsonl,
    Csv,
    #[default]
    Txt,
}
#[cfg(feature = "file_writing")]
impl WriteType {
    pub fn write<W: Write>(
        &self,
        writer: &mut W,
        data: &[Pokemon],
        detail_level: u8,
        pretty: bool,
    ) -> io::Result<()> {
        if data.is_empty() {
            return std::io::Result::Err(io::Error::other("data cant be empty"));
        }

        match self {
            WriteType::Json => {
                //makes it a json array
                writer.write_all("[".as_bytes())?;
                let mut json_string = String::new();
                for pkmn in data {
                    let pkmap = &pkmn.get_as_map(detail_level);
                    let pokemon_string = if pretty {
                        serde_json::to_string_pretty(pkmap)?
                    } else {
                        serde_json::to_string(pkmap)?
                    };
                    json_string += (pokemon_string + ",").as_str();
                    json_string.push('\n');

                }
                //removes the trailing comma
                json_string.pop();
                json_string.pop();
                writer.write_all(json_string.as_bytes())?;
                writer.write_all("]".as_bytes())?;
            }
            //def no copied from json
            WriteType::Jsonl => {
                let mut jsonl_string = String::new();
                for pkmn in data {
                    let pkmap = &pkmn.get_as_map(detail_level);
                    let pokemon_string = if pretty {
                        serde_json::to_string_pretty(pkmap)?
                    } else {
                        serde_json::to_string(pkmap)?
                    };
                    jsonl_string += (pokemon_string + "\n").as_str();
                }
                // no newline at end
                jsonl_string.pop();
                writer.write_all(jsonl_string.as_bytes())?;
            }
            WriteType::Csv => {
                let mut csv_string = String::new();

                for (column_name, _) in &data[0].get_as_vec(detail_level) {
                    csv_string.push_str(column_name);
                    csv_string.push(',');
                }
                csv_string.push('\n');

                for pkmn in data {
                    let vec = pkmn.get_as_vec(detail_level);

                    for (_, column_value) in vec {
                        csv_string.push_str(&column_value);
                        csv_string.push(',');
                    }
                    csv_string.push('\n');
                }
                csv_string = csv_string.replace(",\n", "\n");
                writer.write_all(csv_string.as_bytes())?;
            }
            WriteType::Txt => {
                let mut txt_str = String::new();
                for pokemon in data {
                    for (key, value) in pokemon.get_as_vec(detail_level) {
                        txt_str.push_str(&(key.to_string() + ":" + &value + "\n"));
                    }
                    txt_str.push('\n');
                }
                writer.write_all(txt_str.as_bytes())?;
            }
        }

        Ok(())
    }
}
