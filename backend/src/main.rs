mod app;
mod auth;
mod db;
mod models;
mod errors;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    app::start().await;
}
