use diesel::{Connection, RunQueryDsl, SqliteConnection};

use crate::{
    error::CliError,
    logging::print_order_created,
    models::{
        shipment_packages::ShipmentPackageModel, shipments::ShipmentModel, user::select_all_users,
        user_balances::UserBalance,
    },
    shipmondo::{create_shipment, get_balance},
};

#[inline]
pub fn command(database: &mut SqliteConnection) -> Result<(), CliError> {
    let users = select_all_users(database, false)?;

    if users.is_empty() {
        return Err(CliError::NoUsersFound);
    }

    let selected_user =
        inquire::Select::new("Which account do you wish to use?", users).prompt()?;

    let shipment = create_shipment(
        &selected_user.username,
        &selected_user.password,
        selected_user.production,
    )?;

    let new_balance = get_balance(
        &selected_user.username,
        &selected_user.password,
        selected_user.production,
    )?;

    let mut package_ids = std::collections::HashSet::new();

    for parcel in &shipment.parcels {
        if let Some(pkg_no) = &parcel.pkg_no {
            package_ids.insert(pkg_no.to_string());
        }

        if let Some(pkg_nos) = &parcel.pkg_nos {
            for pkg_no in pkg_nos {
                package_ids.insert(pkg_no.to_string());
            }
        }
    }

    database.transaction(|conn| {
        diesel::insert_into(crate::schema::shipments::table)
            .values(ShipmentModel {
                id: shipment.id,
                user_id: selected_user.id,
                price: shipment.price.clone(),
            })
            .execute(conn)?;

        for package_id in &package_ids {
            diesel::insert_into(crate::schema::shipment_packages::table)
                .values(ShipmentPackageModel {
                    shipment_id: shipment.id,
                    package_id: package_id.to_string(),
                })
                .execute(conn)?;
        }

        diesel::insert_into(crate::schema::user_balances::table)
            .values(UserBalance {
                user_id: selected_user.id,
                amount: new_balance.amount,
                currency_code: new_balance.currency_code.clone(),
            })
            .execute(conn)?;

        diesel::result::QueryResult::Ok(())
    })?;

    print_order_created(
        shipment.id,
        package_ids.into_iter().collect::<Vec<_>>(),
        &shipment.price,
        &new_balance,
    );

    Ok(())
}
