use super::db::DbError;

pub trait RoDbTx
  : Send + Sync
{
  fn commit(self) -> Result<bool, DbError>;
}

pub trait DbTx
  : Send + Sync
{ }