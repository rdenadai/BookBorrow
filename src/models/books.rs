use chrono::{NaiveDateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub year_of_publication: i32,
    pub available: bool,
    #[serde(default = "default_created_at")]
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

fn default_created_at() -> Option<NaiveDateTime> {
    Some(Utc::now().naive_utc())
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::reservations::Entity")]
    Reservation,
}

impl Related<super::reservations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reservation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
