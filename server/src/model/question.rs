use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub options: Vec<String>,
    pub answer: String
}

impl Question {
    const POINTS_IF_CORRECT : u32 = 5;
    const POINTS_IF_INCORRECT : u32 = 0;

    pub fn new(question: String, options: Vec<String>, answer: String) -> Question {
        return Question {
            question,
            options,
            answer
        }
    }

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
            options: self.options.clone(),
            answer: self.answer.clone()
        }
    }
}