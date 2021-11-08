use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub possible_answers: Vec<String>,
    pub answer: String
}
