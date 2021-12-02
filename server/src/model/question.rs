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

    fn uniform_option(&self, option: String) -> String {
        if option.len() == 1 {
            // It's a char, 65 is A in ASCII
            let option_as_byte = option.to_uppercase().as_bytes().to_vec()[0] - 65;
            return self.options.get(option_as_byte as usize).unwrap().clone();
        }

        option
    }

    pub fn get_points_for(&self, option: String) -> u32 {
        let uniform_option = self.uniform_option(option);

        if self.answer == uniform_option {
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