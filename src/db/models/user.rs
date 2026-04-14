use crate::db::schema::users;
use chrono::NaiveDateTime;
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
    pub async fn create(&self, conn: &mut PgConnection) -> Result<usize, Error> {
        insert_into(users::table).values(self).execute(conn)
    }
}
