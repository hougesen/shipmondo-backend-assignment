use crate::{error::CliError, models::user::UserModel};

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

#[inline]
pub fn print_order_created(
    shipment_id: i32,
    package_ids: Vec<String>,
    price: f32,
    new_balance: f32,
) {
    println!(
        "Shipment has been created\nShipment id: {shipment_id}\nPackage ids: {}\nPrice: {price}\nNew account balance: {new_balance}",
        package_ids.join(", "),
    );
}

#[inline]
pub fn print_user_balance(user: &UserModel) {
    println!("The balance of that account is: {}", user.balance);
}
