use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        id::QuizId,
        list::PaginatedList,
        quiz::{
            event::{DeleteQuiz, UpdateQuiz},
            Quiz, QuizListOptions,
        },
    },
    repository::quiz::QuizRepogitory,
};
use shared::error::AppResult;

use crate::database::ConnectionPool;

#[derive(new)]
pub struct QuizRepogitoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl QuizRepogitory for QuizRepogitoryImpl {
    async fn cook_quiz(&self, num: u8) -> AppResult<String> {
        todo!()
    }

    async fn add_quiz(&self, quiz: &str) -> AppResult<()> {
        todo!()
    }

    async fn find_all(&self, option: QuizListOptions) -> AppResult<PaginatedList<Quiz>> {
        todo!()
    }

    async fn find_by_id(&self, quiz_id: QuizId) -> AppResult<Quiz> {
        todo!()
    }

    async fn update_quiz(&self, event: UpdateQuiz) -> AppResult<()> {
        todo!()
    }

    async fn delete_quiz(&self, event: DeleteQuiz) -> AppResult<()> {
        todo!()
    }
}
