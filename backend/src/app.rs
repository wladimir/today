use std::env;

use crate::db::connection::Repo;
use crate::db::repository::Repository;

pub async fn start() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("{}", database_url);

    let repository = Repository(Repo::new(&database_url));
}
