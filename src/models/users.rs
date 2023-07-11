use chrono::{NaiveDateTime, Utc};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub active: bool,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    #[sea_orm(default_value = Utc::now().naive_utc())]
    pub created_at: Option<NaiveDateTime>,
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

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}
