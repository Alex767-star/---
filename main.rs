mod storage;

use storage::{SqliteStorage, Storage};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let storage = SqliteStorage::new("sqlite:hh_analytics.db").await?;
    
    let recent = storage.get_recent_descriptions().await?;
    println!("Fetched {} recent descriptions", recent.len());
    
    Ok(())
}
