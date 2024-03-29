use std::time::Duration;
use super::static_files_generator::BlockRangeOfSegments;

#[derive(Clone)]
pub enum StaticFilesGeneratorEvent {

  Started {
    blockRangeOfSegments: BlockRangeOfSegments
  },

  Finished {
    blockRangeOfSegments: BlockRangeOfSegments,
    timeTaken: Duration
  }
}