CREATE TABLE IF NOT EXISTS vacancies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    employer TEXT NOT NULL,
    description TEXT NOT NULL,
    published_at TIMESTAMP NOT NULL,
    url TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS skill_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL,
    keyword TEXT NOT NULL,
    count INTEGER NOT NULL DEFAULT 0,
    UNIQUE(date, keyword)
);

CREATE INDEX idx_vacancies_published_at ON vacancies(published_at);
CREATE INDEX idx_skill_stats_date ON skill_stats(date);
