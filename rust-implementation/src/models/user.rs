use diesel::{Selectable, prelude::Queryable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserModel {
    pub id: i32,

    pub username: String,

    pub password: String,

    pub production: bool,
}
