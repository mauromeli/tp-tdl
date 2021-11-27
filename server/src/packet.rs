use std::fmt::Error;
use crate::model::kahoot::Kahoot;
use crate::model::player::Player;
use crate::packet::Packets::{ACKCONNECT, ERROR};

use std::string::ToString;
use std::fmt;
pub enum Packets {
    CONNECT(String), //: name
    ACKCONNECT(u8), //: id_player
    ANSWER(u8, String), //(idJugador, respuesta)
    SCORE(),
    ERROR()
}

impl std::fmt::Display for Packets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Packets::ACKCONNECT(player_id) => write!(f, "A,{}", player_id),
            _ => write!(f, "ERROR WHILE FORMATTING PACKET!")
        }
    }
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
    let mut packet_to_send = ERROR();
    match received_packet {
		Packets::CONNECT(name) => {
            let mut player = Player::new(name);
            let player_id = player.id;
            kahoot.add_player(player);
            packet_to_send = ACKCONNECT(player_id);
        },
		Packets::ANSWER(player_id, name) => { /*answer_current_question(player_id, name)*/
            //packet_to_send = "Respuesta (correcta)".parse().unwrap()
        },
		Packets::SCORE() => {
            // /*get_score(name)*/ packet_to_send = "Tenés buen puntaje".parse().unwrap()
        },
        _ => {
            //packet_to_send = "ERROR INESPERADO".parse().unwrap()
        }
    }
    return format!("{}", packet_to_send)
}