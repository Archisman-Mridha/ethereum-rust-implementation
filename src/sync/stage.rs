use async_trait::async_trait;
use ethers_core::types::BlockNumber;
use super::stage_id::StageId;

/*
  The synchronization process is divided into multiple serialized tasks - syncing, validating and
  storing the block headers / download block bodies etc. Each task is handled by a Stage.

  A Stage run starting from the local (outdated) chain tip to the external (new) chain tip.

  The pipeline workflow is somewhat like this :

  1. The HeaderStage is responsible for syncing the block headers, validating the header integrity
  and writing the headers to the database.

  2. Once the HeaderStage completes successfully, the BodyStage will start execution. It downloads
  block bodies for all of the new block headers that were stored locally in the database.

  3. Following a successful BodyStage, the SenderRecoveryStage starts to execute. It's responsible
  for recovering the transaction sender for each of the newly added transactions to the database.

  4. Finally, after all headers, bodies and senders are added to the database, the ExecutionStage
  starts to execute. It's responsible for executing all of the transactions and updating the state
  stored in the database.

  ... Similarly, we have more stages.
*/
#[async_trait]
pub trait Stage<Db>
  : Send + Sync
{
  // Returns the (unique) id of the Stage.
  fn id(&self) -> StageId;

  async fn execute(&mut self,
                   dbTransaction: &mut dyn DbTransaction,
                   input: StageExecutionInput) -> Result<StageExecutionOutput, StageExecutionError>;

  async fn rollback(&mut self,
                    dbTransaction: &mut dyn DbTransaction,
                    input: StageRollbackInput) -> Result<StageRollbackOutput, Box<dyn std::error::Error>>;
}

pub struct StageExecutionInput {
  pub previousStageInfo: Option<(StageId, BlockNumber)>,
  pub blockReachedDuringLastExecution: Option<BlockNumber>
}

pub struct StageExecutionOutput {
  pub blockReached: BlockNumber,
  pub executionCompleted: bool,
  pub reachedChainTip: bool
}

#[derive(Debug, thiserror::Error)]
pub enum StageExecutionError {

  #[error("Stage encountered a state validation error")]
  InvalidState,

  #[error(transparent)]
  Internal(Box<dyn std::error::Error>)
}

pub struct StageRollbackInput {
  pub currentBlock: BlockNumber,
  pub targetBlock: BlockNumber,
  pub responsibleBadBlock: Option<BlockNumber>
}

pub struct StageRollbackOutput {
  pub blockReached: BlockNumber
}

pub trait DbTransaction { }