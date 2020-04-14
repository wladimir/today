use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};

#[derive(Clone)]
pub struct Repo {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl Repo {
    pub fn new(database_url: &str) -> Self {
        Self::from_pool_builder(database_url, r2d2::Builder::default())
    }

    fn from_pool_builder(
        database_url: &str,
        builder: r2d2::Builder<ConnectionManager<PgConnection>>,
    ) -> Self {
        let connection_pool = builder
            .build(ConnectionManager::new(database_url))
            .expect("Error creating connection pool");

        Repo { connection_pool }
    }

    pub fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.connection_pool.get().unwrap()
    }
}
