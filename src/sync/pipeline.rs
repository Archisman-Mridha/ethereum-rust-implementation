use ethers_core::types::U64;
use super::stage::Stage;

/*
  A pipeline executes queued Stages serially.

  An external component determines the current tip of the chain in the network. The pipeline then
  executes each stage in-order. When a stage is executed, it'll run starting from the local
  (outdated) chain tip to the external (new) chain tip.

  In case of a state validation error (determined by the consensus engine) in one of the stages, the
  pipeline will rollback the stages according to their rollback-priority. Stages with higher
  rollback-priority are rolled back first.

  You can also request for a rollback manually.
*/
pub struct Pipeline {
  queuedStages: Vec<QueuedStage>,

  // The pipeline will start by rolling back to this block.
  startWithRollbackToBlock: Option<U64>,

  // The synchronization process will stop after reaching this block.
  stopSyncAfterReachingBlock: Option<U64>,

  // By default, after the entire pipeline has been run, it'll run again. You can disable that by
  // setting this to true.
  exitAfterSync: bool
}

impl Pipeline {
  // Queue a Stage (with rollback-priority = 0) for execution in the pipeline.
  pub fn pushStage<S>(&mut self, stage: S, executionRequiresReachingChainTip: bool) -> &mut Self
    where
      S: Stage + 'static
  {
    self.pushStageWithRollbackPriority(stage, executionRequiresReachingChainTip, 0)
  }

  pub fn pushStageWithRollbackPriority<S>(&mut self,
                                          stage: S,
                                          executionRequiresReachingChainTip: bool,
                                          rollbackPriority: usize) -> &mut Self
    where
      S: Stage + 'static
  {
    let queuedStage= QueuedStage {
      inner: Box::new(stage),
      executionRequiresReachingChainTip,
      rollbackPriority
    };
    self.queuedStages.push(queuedStage);

    self
  }

  pub fn startWithRollbackToBlock(&mut self, startWithRollbackToBlock: Option<U64>) -> &mut Self {
    self.startWithRollbackToBlock= startWithRollbackToBlock;
    self
  }

  pub fn stopSyncAfterReachingBlock(&mut self, stopSyncAfterReachingBlock: Option<U64>) -> &mut Self {
    self.stopSyncAfterReachingBlock= stopSyncAfterReachingBlock;
    self
  }

  pub fn exitAfterSync(&mut self, exitAfterSync: bool) -> &mut Self {
    self.exitAfterSync= exitAfterSync;
    self
  }

  pub async fn run(&mut self) -> Result<( ), Box<dyn std::error::Error>> {
    todo!( )
  }
}

pub struct QueuedStage {
  inner: Box<dyn Stage>,

  // Stages with higher rollback-priority are rolled back first.
  rollbackPriority: usize,

  // Whether the stage execution requires the synchronization process to reach the tip of the chain.
  executionRequiresReachingChainTip: bool
}