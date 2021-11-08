use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    question: String,
    possible_answers: Vec<String>,
    answer: String
}
