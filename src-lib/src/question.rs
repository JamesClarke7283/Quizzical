// question.rs

use serde::{Serialize, Deserialize};
use crate::Answer;

pub trait QuestionType {
    fn prompt(&self) -> &str;
    fn validate_answer(&self, answer: &Answer) -> bool;
    // Add more common methods for question types
}

#[derive(Serialize, Deserialize)]
pub enum Question {
    MultipleChoice(MultipleChoiceQuestion),
    TypedAnswer(TypedAnswerQuestion),
    // Add more question variants as needed
}

impl QuestionType for Question {
    fn prompt(&self) -> &str {
        match self {
            Question::MultipleChoice(q) => q.prompt(),
            Question::TypedAnswer(q) => q.prompt(),
            // Add more match arms for other question variants
        }
    }

    fn validate_answer(&self, answer: &Answer) -> bool {
        match self {
            Question::MultipleChoice(q) => q.validate_answer(answer),
            Question::TypedAnswer(q) => q.validate_answer(answer),
            // Add more match arms for other question variants
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MultipleChoiceQuestion {
    // Multiple choice question-specific fields
}

impl QuestionType for MultipleChoiceQuestion {
    fn prompt(&self) -> &str {
        // Implement prompt for multiple choice question
        "Multiple Choice Question"
    }

    fn validate_answer(&self, answer: &Answer) -> bool {
        // Implement answer validation for multiple choice question
        true
    }
}

#[derive(Serialize, Deserialize)]
pub struct TypedAnswerQuestion {
    // Typed answer question-specific fields
}

impl QuestionType for TypedAnswerQuestion {
    fn prompt(&self) -> &str {
        // Implement prompt for typed answer question
        "Typed Answer Question"
    }

    fn validate_answer(&self, answer: &Answer) -> bool {
        // Implement answer validation for typed answer question
        true
    }
}