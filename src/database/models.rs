#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg(feature = "server")]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::threads)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[cfg(feature = "server")]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::threads)]
pub struct NewThread<'a> {
    pub title: &'a str,
    pub body: &'a str,
}