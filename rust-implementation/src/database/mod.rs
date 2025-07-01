use diesel::Connection;

#[inline]
pub fn connect_to_database() -> Result<diesel::SqliteConnection, diesel::ConnectionError> {
    diesel::SqliteConnection::establish("shipmondo.db")
}
