use axum_core::extract::FromRef;

/// State used by axum to carry the db connection pool.
#[derive(Debug, Clone)]
pub struct State {
    pool: sqlx::PgPool,
}

impl State {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn transaction(&self) -> Result<sqlx::PgTransaction<'static>, sqlx::Error> {
        self.pool.begin().await
    }
}

impl FromRef<State> for sqlx::PgPool {
    fn from_ref(state: &State) -> Self {
        state.pool.clone()
    }
}

