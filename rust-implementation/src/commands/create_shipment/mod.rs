use diesel::{
    Connection, ExpressionMethods, RunQueryDsl, SqliteConnection, query_dsl::methods::FilterDsl,
};

use crate::{
    error::CliError,
    logging::print_order_created,
    models::{
        shipment_packages::ShipmentPackageModel, shipments::ShipmentModel, user::select_all_users,
    },
    shipmondo::create_shipment,
};

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

    let price = shipment.price.parse::<f32>()?;

    let new_balance = selected_user.balance - price;

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
                price: shipment.price,
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

        diesel::update(
            crate::schema::users::table.filter(crate::schema::users::id.eq(selected_user.id)),
        )
        .set(crate::schema::users::balance.eq(new_balance))
        .execute(conn)?;

        diesel::result::QueryResult::Ok(())
    })?;

    print_order_created(
        shipment.id,
        package_ids.into_iter().collect::<Vec<_>>(),
        price,
        new_balance,
    );

    Ok(())
}
