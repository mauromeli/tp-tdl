use crate::packages::Package;

pub fn generate(id_package: &str, params: Vec<String>) -> Result<Package, String> {
    match id_package {
        "C" => generate_connect(params),
        "A" => generate_ack_connect(params),
        "R" => generate_response(params),
        "P" => generate_question(params),
        "T" => generate_finish_game(params),
        _ => Err("Package not valid".to_string()),
    }
}

fn generate_connect(params: Vec<String>) -> Result<Package, String> {
    let player_name = params[0].clone();
    Ok(Package::Connect { player_name })
}

fn generate_ack_connect(params: Vec<String>) -> Result<Package, String> {
    let player_id = params[0].clone();
    Ok(Package::ACKConnect { player_id })
}

fn generate_response(params: Vec<String>) -> Result<Package, String> {
    let player_id = params[0].clone();
    let response = params[1].clone();
    Ok(Package::Response { player_id, response })
}

fn generate_question(params: Vec<String>) -> Result<Package, String> {
    let question = params[0].clone();
    let options = &params[1..];
    let vector = options.to_vec();
    Ok(Package::Question { question, options: vector })
}

fn generate_finish_game(params: Vec<String>) -> Result<Package, String> {
    let player_id = params[0].clone();
    Ok(Package::FinishGame { player_id })
}
