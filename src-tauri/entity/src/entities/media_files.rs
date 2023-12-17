//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "media_files")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub song_id: Option<i32>,
    pub file_name: String,
    pub r#type: String,
    pub weight: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::songs::Entity",
        from = "Column::SongId",
        to = "super::songs::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Songs,
}

impl Related<super::songs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Songs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
