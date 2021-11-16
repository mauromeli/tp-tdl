use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub possible_answers: Vec<String>,
    pub answer: String
}

impl Question {
    const POINTS_IF_CORRECT : u32 = 5;
    const POINTS_IF_INCORRECT : u32 = 0;

    pub fn get_points_for(&self, option: String) -> u32 {
        if self.answer == option {
            Question::POINTS_IF_CORRECT
        } else {
            Question::POINTS_IF_INCORRECT
        }
    }
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