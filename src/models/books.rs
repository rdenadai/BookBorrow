use crate::utils::default::default_created_at;
use chrono::{NaiveDateTime, Utc};
use sea_orm::entity::{prelude::*, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub title: String,
    pub author: String,
    pub year_of_publication: i32,
    pub available: bool,
    #[serde(skip_deserializing)]
    #[serde(default = "default_created_at")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<NaiveDateTime>,
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

impl ActiveModel {
    pub fn merge(&mut self, other: Model) {
        self.title = Set(other.title.to_owned());
        self.author = Set(other.author.to_owned());
        self.year_of_publication = Set(other.year_of_publication.to_owned());
        self.available = Set(other.available.to_owned());
        self.updated_at = Set(Some(Utc::now().naive_utc()));
    }
}
