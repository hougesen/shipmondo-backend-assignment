use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::shipment_packages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ShipmentPackageModel {
    pub package_id: String,

    pub shipment_id: i32,
}
