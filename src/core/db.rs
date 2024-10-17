use anyhow::{Context, Result};
use sqlx::{
    pool::PoolConnection,
    postgres::{PgPool, PgPoolOptions},
    Postgres,
};

#[derive(Debug)]
pub struct PgDb(PgPool);

impl PgDb {
    pub async fn new(connection_uri: &String) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&connection_uri)
            .await
            .context("Failed to connect to PostgreSQL.")?;
        Ok(Self(pool))
    }

    pub fn pool(&self) -> &PgPool {
        &self.0
    }

    pub async fn conn(&self) -> Result<PoolConnection<Postgres>> {
        let conn = self
            .0
            .acquire()
            .await
            .context("Failed to acquire connection.")?;
        Ok(conn)
    }
}
