use crate::packages::Package;
use std::collections::HashMap;

pub fn decode_package(bytes: &[u8]) -> Result<Package, String> {
    match bytes[0] as char {
        'A' => {
            let player_id = std::str::from_utf8(&bytes[1..]).unwrap().to_string();
            Ok(Package::ACKConnect { player_id })
        }
        'P' => {
            let mut string = std::str::from_utf8(&bytes[1..]).unwrap().to_string();
            let mut pos = 1;
            let mut question = "".to_string();
            if let Some(index) = string.find("|") {
                if let Ok(string) = std::str::from_utf8(&(bytes[pos..index + pos]).to_vec()) {
                    question = string.to_string();
                }
                pos = index + pos + 1;
            }
            let mut options = Vec::new();
            for _i in 0..3 {
                string = std::str::from_utf8(&bytes[pos..]).unwrap().to_string();
                if let Some(index) = string.find("-") {
                    if let Ok(string) = std::str::from_utf8(&(bytes[pos..index + pos]).to_vec()) {
                        options.push(string.to_string());
                    }
                    pos = index + pos + 1;
                }
            }
            string = std::str::from_utf8(&bytes[pos..]).unwrap().to_string();
            options.push(string.to_string());
            Ok(Package::Question { question, options })
        }
        'E' => {
            //jugador1,43,jugador2,40,jugador3,30,jugador4,33
            let mut string = std::str::from_utf8(&bytes[1..]).unwrap().to_string();
            let mut pos = 1;
            let mut params = Vec::new();
            for _i in 0..7 {
                if let Some(index) = string.find(",") {
                    if let Ok(string) = std::str::from_utf8(&(bytes[pos..index + pos]).to_vec()) {
                        params.push(string.clone().to_string());
                    }
                    pos = pos + index + 1;
                    string = std::str::from_utf8(&bytes[pos..]).unwrap().to_string();
                }
            }
            string = std::str::from_utf8(&bytes[pos..]).unwrap().to_string();
            params.push(string.to_string());

            let mut players : HashMap<String, String> = HashMap::new();
            for i in (0..params.len()).step_by(2)  {
                players.insert(params[i].clone(), params[i+1].clone());
            }

            Ok(Package::EndGame {
                players
            })
        }
        'W' => {
            let player_id = std::str::from_utf8(&bytes[1..]).unwrap().to_string();
            Ok(Package::Wait { player_id })
        }

        _ => {
            Err("Error parseando el paquete enviado".to_string())
        }
    }
}
