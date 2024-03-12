// lib.rs

pub mod quiz;
pub mod question;

use serde::{Serialize, Deserialize};
use crate::question::Question;

pub trait QuizType {
    fn score(&self, answers: &[Answer]) -> f64;
    // Add more common methods for quiz types
}

#[derive(Serialize, Deserialize)]
pub struct Quiz<T: QuizType> {
    pub title: String,
    pub questions: Vec<Question>,
    pub quiz_type: T,
}

impl<T: QuizType> Quiz<T> {
    pub fn new(title: String, quiz_type: T) -> Self {
        Quiz {
            title,
            questions: Vec::new(),
            quiz_type,
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    // Define the structure of an answer
}