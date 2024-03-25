use async_trait::async_trait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240325_000001_initial_setup"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let users = Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Users::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Users::DiscordId).string().not_null())
            .col(ColumnDef::new(Users::ArcaeaToken).string())
            .to_owned();

        let songs = Table::create()
            .table(Songs::Table)
            .col(
                ColumnDef::new(Songs::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Songs::Title).string().not_null())
            .col(ColumnDef::new(Songs::Artist).string().not_null())
            .col(ColumnDef::new(Songs::ReleaseDate).string().not_null())
            .col(ColumnDef::new(Songs::Pack).string().not_null())
            .to_owned();

        let charts = Table::create()
            .table(Charts::Table)
            .col(
                ColumnDef::new(Charts::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Charts::SongId).integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .name("fk-chart-song_id")
                    .from(Charts::Table, Charts::SongId)
                    .to(Songs::Table, Songs::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .col(ColumnDef::new(Charts::Difficulty).string().not_null())
            .col(ColumnDef::new(Charts::Level).string().not_null())
            .col(ColumnDef::new(Charts::Constant).decimal())
            .col(ColumnDef::new(Charts::Charter).string())
            .to_owned();

        let jackets = Table::create()
            .table(Jackets::Table)
            .col(
                ColumnDef::new(Jackets::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Jackets::SongId).integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .name("fk-jacket-song_id")
                    .from(Jackets::Table, Jackets::SongId)
                    .to(Songs::Table, Songs::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .col(ColumnDef::new(Jackets::ChartId).integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .name("fk-jacket-chart_id")
                    .from(Jackets::Table, Jackets::ChartId)
                    .to(Charts::Table, Charts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .col(ColumnDef::new(Jackets::JacketUrl).string().not_null())
            .to_owned();

        manager.create_table(users).await?;
        manager.create_table(songs).await?;
        manager.create_table(charts).await?;
        manager.create_table(jackets).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let users = Table::drop().table(Users::Table).to_owned();
        let songs = Table::drop().table(Songs::Table).to_owned();
        let charts = Table::drop().table(Charts::Table).to_owned();
        let jackets = Table::drop().table(Jackets::Table).to_owned();

        manager.drop_table(users).await?;
        manager.drop_table(songs).await?;
        manager.drop_table(charts).await?;
        manager.drop_table(jackets).await?;

        Ok(())
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
    Title,
    Artist,
    ReleaseDate,
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
