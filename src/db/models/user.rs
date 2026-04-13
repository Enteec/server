use crate::db::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(uuid))]

pub struct User {
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub password_hash: String,
    pub salt: String,
}

impl User {
    pub fn new() -> Self {
        let now = Utc::now().naive_utc();
        Self {
            uuid: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            name: "".to_string(),
            password_hash: "".to_string(),
            salt: "".to_string(),
        }
    }
}
