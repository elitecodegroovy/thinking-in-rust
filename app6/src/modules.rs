#![allow(unused)]
#![allow(clippy::all)]
use super::schema::users;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
