use ethers_core::types::BlockNumber;
use super::{stage_checkpoint::StageCheckpoint, stage_id::StageId};

pub enum MetricEvent {

  // Stage reached new checkpoint.
  StageReachedCheckpoint {
    stageId: StageId,
    checkpoint: StageCheckpoint,
    knownLatestBlockReachableByStage: Option<BlockNumber>
  }
}