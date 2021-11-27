pub enum Packets {
    CONNECT(String), //: name
    ANSWER(u8, String), //(idJugador, respuesta)
    SCORE(),
    ACKCONNECT(u8), //: id_player
    ERROR()
}

pub fn connect_generator(connect_request: String) -> Packets{
    return  Packets::CONNECT(connect_request.split(",").nth(1).unwrap().to_string())
}

pub fn answer_generator(connect_request: String) -> Packets{
    return  Packets::ANSWER(connect_request.split(",").nth(1).unwrap().parse().unwrap(),
                            connect_request.split(",").nth(2).unwrap().to_string())
}

pub fn score_generator(connect_request: String) -> Packets{
    return  Packets::SCORE()
}

pub fn error_generator(connect_request: String) -> Packets{
    return  Packets::ERROR()
}

//its responsability? Now let's see how we interact w/ model.
//WE SHOULD DO A NEW CLASS.
pub fn command_generator(received_packet: Packets) -> String{
    let mut packet_to_send;
    match received_packet {
		Packets::CONNECT(name) => { /*add_player(name)*/ packet_to_send = "Jug Agregado" },
		Packets::ANSWER(player_id, name) => { /*answer_current_question(player_id, name)*/
            packet_to_send = "Respuesta (correcta)" },
		Packets::SCORE() => { /*get_score(name)*/ packet_to_send = "Tenés buen puntaje" },
        _ => {packet_to_send = "ERROR INESPERADO"}
    }
    return packet_to_send.to_string()
}