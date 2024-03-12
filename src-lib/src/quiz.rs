use super::question::{Question, QuestionType};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RegularQuiz;

pub trait QuizType {
    fn score(&self, answers: Vec<String>, questions: &[Question]) -> f64;
}

impl QuizType for RegularQuiz {
    fn score(&self, answers: Vec<String>, questions: &[Question]) -> f64 {
        let total = questions.len();
        let correct = answers.iter().zip(questions).filter(|(answer, question)| question.validate_answer(answer)).count();

        correct as f64 / total as f64 * 100.0
    }
}
