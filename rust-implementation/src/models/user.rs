use diesel::{
    ExpressionMethods, RunQueryDsl, Selectable, SelectableHelper, SqliteConnection,
    prelude::Queryable,
    query_dsl::methods::{FilterDsl, SelectDsl},
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserModel {
    pub id: i32,

    pub username: String,

    pub password: String,

    pub production: bool,

    #[expect(dead_code)]
    pub is_deleted: bool,
}

#[inline]
pub fn select_all_users(
    database: &mut SqliteConnection,
    include_deleted: bool,
) -> Result<Vec<UserModel>, diesel::result::Error> {
    let q = crate::schema::users::dsl::users.select(UserModel::as_select());

    if include_deleted {
        q.get_results(database)
    } else {
        q.filter(crate::schema::users::is_deleted.eq(false))
            .get_results(database)
    }
}
