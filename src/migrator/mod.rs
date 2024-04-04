use async_trait::async_trait;
use sea_orm_migration::prelude::*;

mod m_20240325_000001_initial_setup;
mod m_20240404_000002_add_version_to_songs;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn sea_orm_migration::prelude::MigrationTrait>> {
        vec![
            Box::new(m_20240325_000001_initial_setup::Migration),
            Box::new(m_20240404_000002_add_version_to_songs::Migration),
        ]
    }
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    DiscordId,
    ArcaeaToken,
}

#[derive(Iden)]
pub enum Songs {
    Table,
    Id,
    IngameId,
    Title,
    Artist,
    ReleaseDate,
    Version,
    Bpm,
    Pack,
}

#[derive(Iden)]
pub enum Charts {
    Table,
    Id,
    SongId,
    Difficulty,
    Level,
    Constant,
    Charter,
}

#[derive(Iden)]
pub enum Jackets {
    Table,
    Id,
    SongId,
    ChartId,
    JacketUrl,
}
