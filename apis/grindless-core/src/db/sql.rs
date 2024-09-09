use std::time::Duration;

use log::{debug, info, warn};
use sea_orm::ConnectionTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbConn, EntityTrait, Schema};

use crate::entities::{account, billing};

pub async fn establish_postgres_connection(
    postgres_url: &String,
) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let mut opt = ConnectOptions::new(postgres_url);
    opt.max_connections(100)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    info!("Connecting to Postgres");

    let db: DatabaseConnection = Database::connect(opt).await?;

    info!("Connected to Postgres");

    create_table_for_entity_if_not_exists(&db, account::Entity).await;
    create_table_for_entity_if_not_exists(&db, billing::Entity).await;
    return Ok(db);
}

pub async fn create_table_for_entity_if_not_exists<E>(db: &DbConn, entity: E)
where
    E: EntityTrait,
{
    warn!("Migrating {}", entity.table_name());
    let builder = db.get_database_backend();
    let my_schema = Schema::new(builder);
    let stmt = builder.build(my_schema.create_table_from_entity(entity).if_not_exists());

    match db.execute(stmt).await {
        Ok(_) => debug!("Migrated: {}", entity.table_name()),
        Err(e) => panic!("Error: {}", e),
    }
}
