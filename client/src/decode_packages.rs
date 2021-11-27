use crate::packages::Package;

pub fn decode_package(bytes: &[u8]) -> Result<Package, String> {
    match bytes[0] as char {
        'A' => {
            let player_id = std::str::from_utf8(&bytes[1..]).unwrap().to_string();
            Ok(Package::ACKConnect { player_id })
        },
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
            for _i in 0..2 {
                string = std::str::from_utf8(&bytes[pos..]).unwrap().to_string();
                if let Some(index) = string.find("-") {
                    if let Ok(string) = std::str::from_utf8(&(bytes[pos..index + pos]).to_vec()) {
                        options.push(string.to_string());
                    }
                    pos = index+pos+1;
                }
            }
            string = std::str::from_utf8(&bytes[pos..]).unwrap().to_string();
            options.push(string.to_string());
            Ok(Package::Question{ question, options })
        },
        _ => {
            let string = std::str::from_utf8(&bytes).unwrap().to_string();
            println!("{}", string);
            Err("Error parseando el paquete enviado".to_string())
        }
    }
}
