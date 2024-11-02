use std::collections::BTreeSet;

use super::id::QuizId;

pub mod event;

#[derive(Debug)]
pub struct Quiz {
    pub quiz_id: QuizId,
    pub question: String,
    pub answer: BTreeSet<Answer>,
}

#[derive(Debug)]
pub struct Answer(String);

#[derive(Debug)]
pub struct QuizListOptions {
    pub limit: i64,
    pub offset: i64,
}
