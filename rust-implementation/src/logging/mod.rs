use crate::{
    error::CliError, models::user_balances::UserBalance, shipmondo::GetBalanceResponseBody,
};

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
    price: &str,
    new_balance: &GetBalanceResponseBody,
) {
    println!(
        "Shipment has been created\nShipment id: {shipment_id}\nPackage ids: {}\nPrice: {price}\nNew account balance: {}{}",
        package_ids.join(", "),
        new_balance.amount,
        new_balance.currency_code
    );
}

#[inline]
pub fn print_user_balance(user: &UserBalance) {
    println!(
        "The balance of that account is: {}{}",
        user.amount, user.currency_code
    );
}
