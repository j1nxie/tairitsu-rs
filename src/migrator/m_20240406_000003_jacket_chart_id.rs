use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use super::{Jackets, Songs};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240406_000003_jacket_chart_id"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Jackets::Table)
            .drop_column(Alias::new("chart_id"))
            .add_column(
                ColumnDef::new(Alias::new("difficulty"))
                    .string()
                    .not_null()
                    .default(""),
            )
            .to_owned();

        manager.alter_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Jackets::Table)
            .drop_column(Alias::new("difficulty"))
            .add_column(ColumnDef::new(Jackets::ChartId).integer().not_null())
            .to_owned();

        manager.alter_table(table).await
    }
}
