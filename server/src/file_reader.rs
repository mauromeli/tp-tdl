use std::fs;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    question: String,
    possible_answers: Vec<String>,
    answer: String
}

pub fn reader() -> Vec<Question> {

    let path = "./questions.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");


    let questions: Vec<Question> = serde_json::from_value(res).unwrap();

    return questions;

}