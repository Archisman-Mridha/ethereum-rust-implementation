use super::transaction::DbTx;

// Helps duplicating tables across databases.
pub trait TableDuplicater: DbTx { }