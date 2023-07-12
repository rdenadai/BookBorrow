use chrono::{NaiveDateTime, Utc};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "reservations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub reservation_date: Option<NaiveDateTime>,
    pub return_date: Option<NaiveDateTime>,
    #[sea_orm(default_value = Utc::now().naive_utc())]
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    Book,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Book.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn merge(&mut self, other: Model) {
        self.user_id = Set(other.user_id.to_owned());
        self.book_id = Set(other.book_id.to_owned());
        self.reservation_date = Set(other.reservation_date.to_owned());
        self.return_date = Set(other.return_date.to_owned());
        self.updated_at = Set(Some(Utc::now().naive_utc()));
    }
}
