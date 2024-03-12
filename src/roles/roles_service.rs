use sqlx::MySqlPool;

pub struct RolesService {
    pool: sqlx::MySqlPool,
}

impl RolesService {
    pub fn new(&self, pool: MySqlPool) -> Self {
        Self { pool }
    }
}