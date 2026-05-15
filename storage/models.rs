use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vacancy {
    pub id: String,
    pub name: String,
    pub employer: String,
    pub description: String,
    pub published_at: DateTime<Utc>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SkillStat {
    pub id: i64,
    pub date: String,
    pub keyword: String,
    pub count: i32,
}
