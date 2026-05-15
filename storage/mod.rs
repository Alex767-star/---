pub mod models;
mod sqlite;

pub use models::{Vacancy, SkillStat};
pub use sqlite::SqliteStorage;

use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migration(String),
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_vacancies(&self, vacancies: &[Vacancy]) -> Result<u64, StorageError>;
    async fn get_recent_descriptions(&self) -> Result<Vec<String>, StorageError>;
    async fn save_skill_stats(&self, stats: &[(String, i32)]) -> Result<(), StorageError>;
}
