use serde::{Serialize, Deserialize};

pub trait QuestionType {
    fn prompt(&self) -> &str;
    fn validate_answer(&self, answer: &str) -> bool;
}

#[derive(Serialize, Deserialize)]
pub enum Question {
    MultipleChoice(MultipleChoiceQuestion),
    TypedAnswer(TypedAnswerQuestion),
}

impl QuestionType for Question {
    fn prompt(&self) -> &str {
        match self {
            Question::MultipleChoice(q) => q.prompt(),
            Question::TypedAnswer(q) => q.prompt(),
        }
    }

    fn validate_answer(&self, answer: &str) -> bool {
        match self {
            Question::MultipleChoice(q) => q.validate_answer(answer),
            Question::TypedAnswer(q) => q.validate_answer(answer),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MultipleChoiceQuestion {
    pub question: String,
    pub possible_answers: Vec<String>,
    pub correct_answer: String,
    pub is_case_sensitive: bool,
}

impl MultipleChoiceQuestion {
    pub fn new(question: String, correct_answer: String, is_case_sensitive: bool) -> Self {
        Self {
            question,
            possible_answers: vec![],
            correct_answer,
            is_case_sensitive,
        }
    }

    pub fn add_answer(&mut self, answer: String) {
        self.possible_answers.push(answer);
    }
}

impl QuestionType for MultipleChoiceQuestion {
    fn prompt(&self) -> &str {
        &self.question
    }

    fn validate_answer(&self, answer: &str) -> bool {
        let answer_check = if self.is_case_sensitive {
            answer.to_string()
        } else {
            answer.to_lowercase()
        };

        let correct_check = if self.is_case_sensitive {
            self.correct_answer.clone()
        } else {
            self.correct_answer.to_lowercase()
        };

        answer_check == correct_check
    }
}

#[derive(Serialize, Deserialize)]
pub struct TypedAnswerQuestion {
    pub question: String,
    pub answer: String,
    pub is_case_sensitive: bool,
}

impl TypedAnswerQuestion {
    pub fn new(question: String, answer: String, is_case_sensitive: bool) -> Self {
        Self {
            question,
            answer,
            is_case_sensitive,
        }
    }
}

impl QuestionType for TypedAnswerQuestion {
    fn prompt(&self) -> &str {
        &self.question
    }

    fn validate_answer(&self, answer: &str) -> bool {
        let answer_check = if self.is_case_sensitive {
            answer.to_string()
        } else {
            answer.to_lowercase()
        };

        let correct_check = if self.is_case_sensitive {
            self.answer.clone()
        } else {
            self.answer.to_lowercase()
        };

        answer_check == correct_check
    }
}
