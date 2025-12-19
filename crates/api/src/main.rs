use std::process::ExitCode;
use tracing_subscriber::{fmt, EnvFilter};
use shared::config::Settings;

mod http;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = run().await {
        eprintln!("fatal: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());

    if format.eq_ignore_ascii_case("json") {
        fmt().with_env_filter(filter).json().init();
    } else {
        fmt().with_env_filter(filter).pretty().init();
    }
}

fn init_settings() -> Settings {
    let settings = match Settings::load() {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("error loading config: {err}");
            std::process::exit(1);
        }
    };

    settings
}

fn init_modules(settings: Settings) {

}

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Staring application");
    init_tracing();
    let settings: Settings = init_settings();

    let app = http::router();

    tracing::info!("starting api");

    let bind_addr = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    let addr = listener.local_addr()?;

    println!("listening on http://{bind_addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
