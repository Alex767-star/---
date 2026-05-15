use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};
use super::{Storage, StorageError, Vacancy};

pub struct PgStorage {
    pool: PgPool,
}

impl PgStorage {
    pub async fn new(database_url: &str) -> Result<Self, StorageError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        
        sqlx::query(include_str!("../../migrations/001_init.sql"))
            .execute(&pool)
            .await?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl Storage for PgStorage {
    async fn save_vacancies(&self, vacancies: &[Vacancy]) -> Result<u64, StorageError> {
        let mut inserted = 0u64;
        
        for vacancy in vacancies {
            let result = sqlx::query(
                r#"
                INSERT INTO vacancies (id, name, employer, description, published_at, url)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT(id) DO UPDATE SET
                    name = EXCLUDED.name,
                    employer = EXCLUDED.employer,
                    description = EXCLUDED.description,
                    published_at = EXCLUDED.published_at,
                    url = EXCLUDED.url
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
            
            inserted += result.rows_affected();
        }
        
        Ok(inserted)
    }

    async fn get_recent_descriptions(&self) -> Result<Vec<String>, StorageError> {
        let descriptions = sqlx::query_scalar::<_, String>(
            r#"
            SELECT description 
            FROM vacancies 
            WHERE published_at >= NOW() - INTERVAL '1 day'
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
                VALUES ($1, $2, $3)
                ON CONFLICT(date, keyword) DO UPDATE SET
                    count = EXCLUDED.count
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
