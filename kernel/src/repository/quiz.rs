use async_trait::async_trait;
use mockall::automock;
use shared::error::AppResult;

use crate::model::{
    id::QuizId,
    list::PaginatedList,
    quiz::{
        event::{DeleteQuiz, UpdateQuiz},
        Quiz, QuizListOptions,
    },
};

#[automock]
#[async_trait]
pub trait QuizRepogitory: Send + Sync {
    async fn cook_quiz(&self, num: u8) -> AppResult<String>;
    async fn add_quiz(&self, quiz: &str) -> AppResult<()>;
    async fn find_all(&self, option: QuizListOptions) -> AppResult<PaginatedList<Quiz>>;
    async fn find_by_id(&self, quiz_id: QuizId) -> AppResult<Quiz>;
    async fn update_quiz(&self, event: UpdateQuiz) -> AppResult<()>;
    async fn delete_quiz(&self, event: DeleteQuiz) -> AppResult<()>;
}
