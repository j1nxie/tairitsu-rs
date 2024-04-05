use dotenvy::dotenv;
use migrator::Migrator;
use poise::serenity_prelude as serenity;
use reqwest::Client;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod commands;
mod constants;
mod migrator;
mod models;
mod utils;

struct Data {
    db: DatabaseConnection,
    client: Client,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().expect("failed to load .env file.");
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let db = Database::connect(db_url).await?;

    Migrator::up(&db, None).await?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::status::status(),
                commands::profile::profile(),
                commands::profile::login(),
                commands::profile::logout(),
                commands::recent::recent(),
                commands::help::help(),
                commands::song::song(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("a>".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { db, client })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

    Ok(())
}
