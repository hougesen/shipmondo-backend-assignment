use diesel::Connection;
use diesel::{RunQueryDsl, SelectableHelper, SqliteConnection, prelude::Insertable};

use crate::error::CliError;
use crate::logging::print_user_has_been_inserted;
use crate::models::user::UserModel;
use crate::models::user_balances::UserBalance;
use crate::shipmondo::{GetBalanceResponseBody, get_balance};

#[derive(Insertable)]
#[diesel(table_name  = crate::schema::users )]
struct NewUser {
    username: String,

    password: String,

    production: bool,
}

impl NewUser {
    pub fn insert_new_user(
        &self,
        database: &mut SqliteConnection,
        balance: GetBalanceResponseBody,
    ) -> Result<UserModel, diesel::result::Error> {
        database.transaction(|conn| {
            let u = diesel::insert_into(crate::schema::users::table)
                .values(self)
                .returning(UserModel::as_returning())
                .get_result(conn)?;

            diesel::insert_into(crate::schema::user_balances::table)
                .values(UserBalance {
                    user_id: u.id,
                    amount: balance.amount,
                    currency_code: balance.currency_code,
                })
                .execute(conn)?;

            Ok(u)
        })
    }
}

#[inline]
fn prompt_auth_user() -> Result<String, inquire::InquireError> {
    inquire::Text::new("What is the username?")
        .with_validator(inquire::validator::MinLengthValidator::new(1))
        .prompt()
}

#[inline]
fn prompt_auth_key() -> Result<String, inquire::InquireError> {
    inquire::Text::new("What is the password?")
        .with_validator(inquire::validator::MinLengthValidator::new(1))
        .prompt()
}

#[inline]
fn prompt_production() -> Result<bool, inquire::InquireError> {
    inquire::Confirm::new("Should the user be used in production?")
        .with_default(false)
        .prompt()
}

#[inline]
fn prompt_new_user() -> Result<NewUser, CliError> {
    let username = prompt_auth_user()?;
    let password = prompt_auth_key()?;
    let production = prompt_production()?;

    Ok(NewUser {
        username,
        password,
        production,
    })
}

#[inline]
pub fn command(database: &mut SqliteConnection) -> Result<(), CliError> {
    let user = prompt_new_user()?;

    let balance = get_balance(&user.username, &user.password, user.production)?;

    user.insert_new_user(database, balance)?;

    print_user_has_been_inserted();

    Ok(())
}
