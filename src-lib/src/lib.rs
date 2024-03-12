pub mod quiz;
pub mod question;
pub mod score; // Assuming we're going to create this module for different scoring mechanisms

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Quiz<T: quiz::QuizType> {
    pub title: String,
    pub questions: Vec<question::Question>,
    pub quiz_type: T,
}

impl<T: quiz::QuizType> Quiz<T> {
    pub fn new(title: String, quiz_type: T) -> Self {
        Quiz {
            title,
            questions: Vec::new(),
            quiz_type,
        }
    }

    pub fn add_question(&mut self, question: question::Question) {
        self.questions.push(question);
    }
}
