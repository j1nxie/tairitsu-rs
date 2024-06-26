//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "songs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub ingame_id: String,
    pub title: String,
    pub artist: String,
    pub release_date: DateTime,
    pub bpm: String,
    pub pack: String,
    pub version: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::charts::Entity")]
    Charts,
    #[sea_orm(has_many = "super::jackets::Entity")]
    Jackets,
}

impl Related<super::charts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Charts.def()
    }
}

impl Related<super::jackets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Jackets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
