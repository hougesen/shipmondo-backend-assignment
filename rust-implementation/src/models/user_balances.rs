use diesel::{
    ExpressionMethods, QueryDsl, RunQueryDsl, Selectable, SelectableHelper, SqliteConnection,
    prelude::{Insertable, Queryable},
};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user_balances)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserBalance {
    pub user_id: i32,

    pub amount: f32,

    pub currency_code: String,
}

#[inline]
pub fn get_user_balance(
    database: &mut SqliteConnection,
    user_id: i32,
) -> Result<UserBalance, diesel::result::Error> {
    crate::schema::user_balances::dsl::user_balances
        .select(UserBalance::as_select())
        .filter(crate::schema::user_balances::user_id.eq(user_id))
        .order_by(crate::schema::user_balances::timestamp.desc())
        .get_result(database)
}
