mod app;
mod auth;
mod db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    app::start().await;
}
