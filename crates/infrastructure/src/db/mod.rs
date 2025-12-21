pub mod entities;
pub mod mapper;
pub mod repos;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn init_db(
    settings: &shared::config::DatabaseSettings,
) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(&settings.url).await?;
    Ok(db)
}
