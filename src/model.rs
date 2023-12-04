use super::schema::students;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = students)]
pub struct NewStudent<'a> {
    pub name: &'a str,
    pub body: &'a str,
}