# hh-analyzer — Storage Layer

Слой хранения данных для анализа вакансий с hh.ru. Реализован через трейты с возможностью подмены реализации.

## Архитектура
src/
└── storage/
    ├── mod.rs       # Trait Storage + StorageError
    ├── models.rs    # Vacancy, SkillStat
    ├── sqlite.rs    # SqliteStorage
    └── postgres.rs  # PgStorage
migrations/
└── 001_init.sql     # DDL


## Стек

- Rust 2021 edition
- sqlx 0.7 (SQLite + PostgreSQL)
- async-trait
- chrono
- thiserror

## Таблицы

### vacancies
| Поле | Тип | Назначение |
|------|-----|------------|
| id | TEXT PK | ID вакансии с hh.ru |
| name | TEXT | Название |
| employer | TEXT | Компания |
| description | TEXT | Полный текст |
| published_at | TIMESTAMP | Дата публикации |
| url | TEXT | Ссылка |

### skill_stats
| Поле | Тип | Назначение |
|------|-----|------------|
| id | INTEGER PK | Автоинкремент |
| date | DATE | День статистики |
| keyword | TEXT | Ключевое слово |
| count | INTEGER | Количество упоминаний |

UNIQUE(date, keyword) — идемпотентный UPSERT.

## Использование
use storage::{SqliteStorage, Storage};

let storage = SqliteStorage::new("sqlite:hh_analytics.db?mode=rwc").await?;
storage.save_vacancies(&vacancies).await?;
let descriptions = storage.get_recent_descriptions().await?;
storage.save_skill_stats(&stats).await?;


## Переключение на PostgreSQL
use storage::PgStorage;
let storage = PgStorage::new("postgresql://user:pass@localhost/hh").await?;

Трейт `Storage` един для обеих реализаций — достаточно поменять тип в точке инициализации.

## Сборка
cargo build
cargo run
