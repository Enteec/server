use crate::db::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, result::Error};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
    pub fn new(name: &str, password: &str, salt: &str) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            uuid: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            name: name.to_string(),
            password_hash: password.to_string(),
            salt: salt.to_string(),
        }
    }
}

impl User {
    pub async fn create(&self, conn: &mut PgConnection) -> Result<usize, Error> {
        insert_into(users::table).values(self).execute(conn)
    }
}
