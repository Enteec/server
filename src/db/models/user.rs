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
    pub email: String,
    pub password_hash: String,
    pub salt: String,
}

impl User {
    pub fn new(name: &str, email: &str, password_hash: &str, salt: &str) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            uuid: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            name: name.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            salt: salt.to_string(),
        }
    }
}

impl User {
    pub async fn create(&self, conn: &mut PgConnection) -> Result<usize, Error> {
        insert_into(users::table).values(self).execute(conn)
    }

    pub async fn find_by_name(name: &str, conn: &mut PgConnection) -> Result<Option<User>, Error> {
        users::table
            .filter(users::name.eq(name))
            .first::<Self>(conn)
            .optional()
    }
}
