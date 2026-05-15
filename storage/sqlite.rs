use async_trait::async_trait;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use super::{Storage, StorageError, Vacancy};

pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(database_url: &str) -> Result<Self, StorageError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        sqlx::query(include_str!("../../migrations/001_init.sql"))
            .execute(&pool)
            .await?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn save_vacancies(&self, vacancies: &[Vacancy]) -> Result<u64, StorageError> {
        let mut inserted = 0u64;
        
        for vacancy in vacancies {
            let result = sqlx::query(
                r#"
                INSERT INTO vacancies (id, name, employer, description, published_at, url)
                VALUES (?, ?, ?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    employer = excluded.employer,
                    description = excluded.description,
                    published_at = excluded.published_at,
                    url = excluded.url
                "#
            )
            .bind(&vacancy.id)
            .bind(&vacancy.name)
            .bind(&vacancy.employer)
            .bind(&vacancy.description)
            .bind(vacancy.published_at)
            .bind(&vacancy.url)
            .execute(&self.pool)
            .await?;
            
            if result.rows_affected() > 0 {
                inserted += 1;
            }
        }
        
        Ok(inserted)
    }

    async fn get_recent_descriptions(&self) -> Result<Vec<String>, StorageError> {
        let descriptions = sqlx::query_scalar::<_, String>(
            r#"
            SELECT description 
            FROM vacancies 
            WHERE published_at >= datetime('now', '-1 day')
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(descriptions)
    }

    async fn save_skill_stats(&self, stats: &[(String, i32)]) -> Result<(), StorageError> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        for (keyword, count) in stats {
            sqlx::query(
                r#"
                INSERT INTO skill_stats (date, keyword, count)
                VALUES (?, ?, ?)
                ON CONFLICT(date, keyword) DO UPDATE SET
                    count = excluded.count
                "#
            )
            .bind(&today)
            .bind(keyword)
            .bind(count)
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }
}
