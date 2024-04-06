use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "charts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub song_id: String,
    pub difficulty: String,
    pub level: String,
    pub constant: Decimal,
    pub charter: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::songs::Entity",
        from = "Column::SongId",
        to = "super::songs::Column::IngameId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Songs,
}

impl Related<super::songs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Songs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
