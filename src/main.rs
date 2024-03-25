use dotenvy::dotenv;
use migrator::Migrator;
use poise::serenity_prelude as serenity;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod commands;
mod constants;
mod migrator;
mod models;

struct Data {
    db: DatabaseConnection,
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
    let intents = serenity::GatewayIntents::non_privileged();

    let db = Database::connect(db_url).await?;

    tracing::info!("no");

    Migrator::up(&db, None).await?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::status::status()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { db })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

    Ok(())
}
