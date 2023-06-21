use api::{app::App, settings::Settings};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    let settings = Settings::new().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(Level::from(settings.log_level))
        .finish()
        .init();

    let app = App::new(&settings).await;

    app.run().await.unwrap();
}
