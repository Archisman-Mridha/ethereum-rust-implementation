use ethers_core::types::BlockNumber;
use super::{stage::{StageExecutionOutput, StageRollbackInput, StageRollbackOutput}, stage_id::StageId};

pub enum PipelineEvent {
  // Emmitted right before running a Stage.
  Running {
    stageId: StageId,
    blockReachedDuringPreviousExecution: Option<BlockNumber>
  },

  // Emmitted when a Stage has just finished running.
  Ran {
    stageId: StageId,
    result: Option<StageExecutionOutput>
  },

  // Emmitted right before a stage is about to rollback.
  Rollbacking {
    stageId: StageId,
    result: Option<StageRollbackInput>
  },

  // Emmitted right after a stage has rollbacked.
  Rollbacked {
    stageId: StageId,
    result: Option<StageRollbackOutput>
  }
}