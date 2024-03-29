use std::ops::RangeInclusive;
use ethers_core::types::BlockNumber;

pub enum SegmentType {
  BlockHeaders,
  Transactions,
  TransactionReceipts
}

pub trait Segment
  : Send + Sync
{
  // Copies segment (within the given block-range) to static files.
  fn copyToStaticFiles(&self,
                       blockRange: RangeInclusive<BlockNumber>);
}