use diesel::SqliteConnection;

use crate::logging::print_user_balance;
use crate::models::user_balances::get_user_balance;
use crate::{error::CliError, models::user::select_all_users};

#[inline]
pub fn command(database: &mut SqliteConnection) -> Result<(), CliError> {
    let users = select_all_users(database, false)?;

    let selected_user =
        inquire::Select::new("Which user do you wish to check balance of?", users).prompt()?;

    let balance_history = get_user_balance(database, selected_user.id)?;

    print_user_balance(&balance_history);

    Ok(())
}
