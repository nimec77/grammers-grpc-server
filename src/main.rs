mod logger;
mod app_config;

#[tokio::main]
async fn main() {
    logger::init_logger();
    let config = app_config::AppConfig::load_config().expect("Failed to load config");
}
