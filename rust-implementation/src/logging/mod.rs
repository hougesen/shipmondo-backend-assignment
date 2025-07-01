use crate::error::CliError;

#[inline]
pub fn log_error(error: &CliError) {
    eprintln!("{error}");
}

#[inline]
pub fn print_user_has_been_inserted() {
    println!("User has been saved!");
}

#[inline]
pub fn print_user_has_been_deleted() {
    println!("User has been deleted!");
}
