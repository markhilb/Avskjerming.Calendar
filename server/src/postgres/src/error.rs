#[derive(Debug)]
pub enum PostgresError {
    Connection,
    Transaction,
    Query,
    DataConversion,
}

impl std::error::Error for PostgresError {}

impl std::fmt::Display for PostgresError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresError::Connection => f.write_str("failed to acquire a database connection"),
            PostgresError::Transaction => f.write_str("failed to start/commit transaction"),
            PostgresError::Query => f.write_str("a query related error occured"),
            PostgresError::DataConversion => {
                f.write_str("failed to convert data to postgres specific data type")
            }
        }
    }
}
