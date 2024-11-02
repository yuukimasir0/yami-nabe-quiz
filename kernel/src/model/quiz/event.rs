use crate::model::id::QuizId;

use super::{Answer, Quiz};

#[derive(Debug)]
pub struct CreateQuiz {
    pub quiz: String,
    pub answer: Vec<String>,
}

#[derive(Debug)]
pub struct UpdateQuiz {
    pub quiz: Quiz,
    pub answer: Vec<Answer>,
}

#[derive(Debug)]
pub struct DeleteQuiz {
    pub quiz_id: QuizId,
}
