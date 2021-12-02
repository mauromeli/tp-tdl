use crate::model::question::Question;
use std::fs;
use serde_json;

pub fn reader() -> Vec<Question> {
    let path = "questions.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

    let questions: Vec<Question> = serde_json::from_value(res).unwrap();

    return questions;
}