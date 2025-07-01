use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::shipments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ShipmentModel {
    /// Shipment id from shipmondo
    pub id: i32,

    pub user_id: i32,

    pub price: String,
}
