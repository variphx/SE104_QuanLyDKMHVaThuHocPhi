use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct Context {
    db_pool: Arc<PgPool>,
}

impl Context {
    pub fn new(db_pool: PgPool) -> Self {
        let db_pool = Arc::new(db_pool);
        Self { db_pool }
    }
}

impl Context {
    pub fn pool(&self) -> &PgPool {
        &self.db_pool
    }
}
