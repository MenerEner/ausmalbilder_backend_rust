use crate::http::state::AppState;
use shared::config::Settings;
use std::process::ExitCode;
use tracing_subscriber::{EnvFilter, fmt};

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
    match Settings::load() {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("error loading config: {err}");
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();
    tracing::info!("Starting application");
    let settings: Settings = init_settings();
    tracing::info!(env = %settings.env, server_host = %settings.server.host, server_port = %settings.server.port, "configuration loaded");
    tracing::debug!(?settings, "detailed configuration");

    tracing::info!("initializing database");
    let db = infrastructure::db::init_db(&settings.database).await?;

    // Initialize dependencies
    let user_repo = std::sync::Arc::new(infrastructure::db::repos::PostgresUserRepository::new(
        db.clone(),
    ));
    let token_repo = std::sync::Arc::new(
        infrastructure::db::repos::PostgresEmailVerificationTokenRepository::new(db.clone()),
    );
    let password_hasher = std::sync::Arc::new(infrastructure::security::Argon2Hasher);
    let email_service: std::sync::Arc<dyn application::ports::email_service::EmailService> =
        std::sync::Arc::new(infrastructure::email::MailtrapEmailService::new(
            settings.mailtrap.clone(),
        ));

    // Initialize use cases
    let create_user_use_case =
        application::use_cases::CreateUserUseCase::new(user_repo.clone(), password_hasher.clone());
    let get_user_use_case = application::use_cases::GetUserUseCase::new(user_repo.clone());
    let delete_user_use_case = application::use_cases::DeleteUserUseCase::new(user_repo.clone());
    let list_users_use_case = application::use_cases::ListUsersUseCase::new(user_repo.clone());
    let signup_use_case = application::use_cases::SignupUseCase::new(
        user_repo.clone(),
        token_repo.clone(),
        password_hasher,
        email_service,
    );
    let verify_email_use_case =
        application::use_cases::VerifyEmailUseCase::new(user_repo, token_repo);

    let state = AppState::new(
        db,
        create_user_use_case,
        get_user_use_case,
        delete_user_use_case,
        list_users_use_case,
        signup_use_case,
        verify_email_use_case,
    );
    let app = http::router(state);

    tracing::info!("starting api");

    let bind_addr = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    println!("listening on http://{bind_addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
