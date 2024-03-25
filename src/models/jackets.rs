use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "jackets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub song_id: i32,
    pub chart_id: i32,
    pub jacket_url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::charts::Entity",
        from = "Column::ChartId",
        to = "super::charts::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Charts,
    #[sea_orm(
        belongs_to = "super::songs::Entity",
        from = "Column::SongId",
        to = "super::songs::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Songs,
}

impl Related<super::charts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Charts.def()
    }
}

impl Related<super::songs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Songs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
