use sqlx::{SqlitePool, migrate::Migrator};

use crate::Result;

pub mod template;

pub static MIGRATOR: Migrator = sqlx::migrate!();

pub struct State {
    pool: SqlitePool,
}

#[derive(Default)]
pub struct StateBuilder {
    pool: Option<SqlitePool>,
}

impl StateBuilder {
    pub fn with_pool(self, pool: SqlitePool) -> Self {
        let pool_option = Some(pool);

        Self { pool: pool_option }
    }

    pub async fn build(self) -> Result<State> {
        let pool = if let Some(pool) = self.pool {
            pool
        } else {
            SqlitePool::connect("file::memory:?cache=shared").await?
        };

        Ok(State { pool })
    }
}
