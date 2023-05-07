//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "workflow")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub build: i32,
    pub name: String,
    pub run_id: Option<i64>,
    pub url: String,
    pub status: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::build::Entity",
        from = "Column::Build",
        to = "super::build::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Build,
}

impl Related<super::build::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Build.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}