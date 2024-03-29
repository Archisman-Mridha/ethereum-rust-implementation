use super::{table::TableDuplicater, transaction::{DbTx, RoDbTx}};

// Can open read-only and read-writeable transactions.
pub trait Db
  : Send + Sync + Sized
{
  type RoDbTx: RoDbTx + 'static;
  type DbTx: RoDbTx + DbTx + TableDuplicater + 'static;

  // Creates and returns a read-only database transaction.
  fn roDbTx(&self) -> Result<Self::RoDbTx, DbError>;

  // Creates and returns a read-writeable database transaction.
  fn dbTx(&self) -> Result<Self::DbTx, DbError>;

  // Executes a function by createing and passing a read-only transaction to it. It's ensured that
  // the transaction is closed after the function execution.
  fn withRoDbTx<T, F>(&self, f: F) -> Result<T, DbError>
    where
      F: FnOnce(&Self::RoDbTx) -> T
  {
    let roDbTx= self.roDbTx( )?;
    let fnExecutionResult= f(&roDbTx);
    roDbTx.commit( )?;

    Ok(fnExecutionResult)
  }

  // Executes a function by createing and passing a read-writeable transaction to it. It's ensured
  // that the transaction is closed after the function execution.
  fn withDbTx<T, F>(&self, f: F) -> Result<T, DbError>
    where
      F: FnOnce(&Self::DbTx) -> T
  {
    let dbTx= self.dbTx( )?;
    let fnExecutionResult= f(&dbTx);
    dbTx.commit( )?;

    Ok(fnExecutionResult)
  }
}

pub enum DbError { }