use crate::model::kahoot::Kahoot;
use crate::model::player::Player;

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
pub fn command_generator(received_packet: Packets, kahoot: &mut Kahoot) -> String{
    let mut packet_to_send;
    match received_packet {
		Packets::CONNECT(name) => {
            kahoot.add_player(Player::new(name));
            packet_to_send = kahoot.get_winner().0.to_string(); //NOT WINNER, JUST ID!

        },
		Packets::ANSWER(player_id, name) => { /*answer_current_question(player_id, name)*/
            packet_to_send = "Respuesta (correcta)".parse().unwrap()
        },
		Packets::SCORE() => { /*get_score(name)*/ packet_to_send = "Tenés buen puntaje".parse().unwrap() },
        _ => {packet_to_send = "ERROR INESPERADO".parse().unwrap() }
    }
    return packet_to_send.to_string()
}