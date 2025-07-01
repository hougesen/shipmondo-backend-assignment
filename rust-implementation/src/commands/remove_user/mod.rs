use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl, SqliteConnection};

use crate::logging::print_user_has_been_deleted;
use crate::schema;
use crate::{
    error::CliError,
    models::user::{UserModel, select_all_users},
};

impl core::fmt::Display for UserModel {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}:{} ({})",
            self.username,
            self.password,
            if self.production {
                "production"
            } else {
                "development"
            }
        )
    }
}

#[inline]
pub fn command(database: &mut SqliteConnection) -> Result<(), CliError> {
    let users = select_all_users(database, false)?;

    if users.is_empty() {
        return Err(CliError::NoUsersFound);
    }

    let selected_user = inquire::Select::new("Which user should be deleted?", users).prompt()?;

    diesel::update(schema::users::table.filter(schema::users::id.eq(selected_user.id)))
        .set(schema::users::is_deleted.eq(true))
        .get_result::<UserModel>(database)?;

    print_user_has_been_deleted();

    Ok(())
}
