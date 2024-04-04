use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use super::Songs;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240404_000002_add_version_to_songs"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Songs::Table)
            .add_column(ColumnDef::new(Songs::Version).string().not_null())
            .to_owned();

        manager.alter_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Songs::Table)
            .drop_column(Songs::Version)
            .to_owned();

        manager.alter_table(table).await
    }
}
