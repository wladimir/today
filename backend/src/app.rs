use std::env;

use crate::db::connection::Repo;

pub async fn start() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("{}", database_url);

    let repository = Repository(Repo::new(&database_url));
}
