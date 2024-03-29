#![allow(non_snake_case)]

/*
  Data that has reached a finalized state and won't undergo further changes (essentially frozen)
  should be read without concerns of modification. This makes it unsuitable for traditional databases.

  Such type of data will be copied from the current database to multiple static files (with custom
  file format called 'NippyJar'), aggregated by block ranges. At every 500_000th block new static
  files are created.

  Before getting copied, the data gets divided into multiple categories - e.g. block headers /
  transactions / transaction receipts etc. Data contained in each category is called a 'segment'.
*/

pub mod segment;
pub mod block_headers_segment;
pub mod transactions_segment;
pub mod transaction_receipts_segment;
pub mod nippy_jar;
pub mod static_files_handler;
pub mod static_files_generator;
pub mod static_files_generator_event;