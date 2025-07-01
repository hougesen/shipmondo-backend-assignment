use diesel::{
    ExpressionMethods, RunQueryDsl, Selectable, SelectableHelper, SqliteConnection,
    prelude::Queryable,
    query_dsl::methods::{FilterDsl, SelectDsl},
};

use crate::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserModel {
    pub id: i32,

    pub username: String,

    pub password: String,

    pub production: bool,

    pub is_deleted: bool,

    pub balance: f32,
}

#[inline]
pub fn select_all_users(
    database: &mut SqliteConnection,
    include_deleted: bool,
) -> Result<Vec<UserModel>, diesel::result::Error> {
    let q = schema::users::dsl::users.select(UserModel::as_select());

    if include_deleted {
        q.get_results(database)
    } else {
        q.filter(schema::users::is_deleted.eq(false))
            .get_results(database)
    }
}
