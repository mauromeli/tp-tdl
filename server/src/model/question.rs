use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub possible_answers: Vec<String>,
    pub answer: String
}

impl Clone for Question{
    fn clone(&self) -> Self {
        return Question{
            question: self.question.clone(),
            possible_answers: self.possible_answers.clone(),
            answer: self.answer.clone()
        }
    }
}