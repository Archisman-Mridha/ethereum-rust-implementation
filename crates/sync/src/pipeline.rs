use ethers_core::types::BlockNumber;
use tokio::sync::mpsc::UnboundedSender;
use db::interfaces;
use super::{
  metrics::MetricEvent, pipeline_control_flow::PipelineControlFlow, pipeline_error::PipelineError,
  pipeline_event::PipelineEvent, stage::Stage
};
use tracing::warn;

/*
  A pipeline executes queued Stages serially.

  An external component determines the current tip of the chain in the network. The pipeline then
  executes each stage in-order. When a stage is executed, it'll run starting from the local
  (outdated) chain tip to the external (new) chain tip.

  In case of a state validation error (determined by the consensus engine) in one of the stages, the
  pipeline will rollback the stages in reverse order of execution.
  It's also possible to manually request for a rollback.

  After the entire pipeline has been run, it will run again unless asked to stop (see
  self.stopSyncAfterReachingBlock).
*/
pub struct Pipeline<Db>
  where
    Db: interfaces::db::Db
{

  stages: Vec<Box<dyn Stage<Db>>>,

  // The synchronization process will stop after reaching this block.
  stopSyncAfterReachingBlock: Option<BlockNumber>,

  eventEmitters: Vec<UnboundedSender<PipelineEvent>>,
  metricEventsEmitter: Option<UnboundedSender<MetricEvent>>
}

impl<Db> Pipeline<Db>
  where
    Db: interfaces::db::Db
{
  // Runs the pipeline.
  pub async fn run(&mut self) -> Result<( ), PipelineError> {
    let _= self.emitProgressMetricOfStages( );

    loop {
      let controlFlow= self.runOnce( ).await?;

      // Terminate the pipeline if block represented by self.stopSyncAfterReachingBlock is reached.
      todo!( )
    }
  }

  /*
    Runs the pipeline once.

    After successfull execution of a Stage, a database commit is performed.

    If any stage is unsuccessfull at execution, then the rollback process is started. This will undo
    the progress across the entire pipeline up to the block that caused the error.
  */
  pub async fn runOnce(&mut self) -> Result<PipelineControlFlow, PipelineError> {
    unimplemented!( )
  }

  async fn executeStage(&mut self,
                        blockReachedByPreviousStage: Option<BlockNumber>,
                        stageIndex: usize)
  {
    let stageCount= self.stages.len( );

    let stage= &mut self.stages[stageIndex];
    let stageId= stage.id( );

    let mut stageMadeProgress= false;

    let stopSyncAfterReachingBlock= self.stopSyncAfterReachingBlock
                                      .or(blockReachedByPreviousStage);

    loop {
      let previousStageCheckpoint= todo!( );

      let stopSync= todo!("Check whether the stage has reached 'stopSyncAfterReachingBlock' or not");
      if stopSync {
        // TODO: Add 'blockReachedByPreviousStageCheckpoint' to the tracing event.
        warn!(
          target: "sync::pipeline",
          stage= %stageId,
          stopSyncAfterReachingBlock= self.stopSyncAfterReachingBlock,
          "Skipping Stage, since it reached 'stopSyncAfterReachingBlock'"
        );

        todo!( )
      }

      unimplemented!( )
    }
  }
}

impl<Db> Pipeline<Db>
  where
    Db: interfaces::db::Db
{
  // Emits metrics related to progress of each stage in the pipeline.
  pub fn emitProgressMetricOfStages(&mut self) -> Result<( ), PipelineError> {
    if self.metricEventsEmitter.is_none( ) {
      return Ok(( ));
    }

    let metricEventsEmitter= self.metricEventsEmitter.as_ref( ).unwrap( );

    for stage in &self.stages {
      let stageId= stage.id( );

      let _= metricEventsEmitter.send(MetricEvent::StageReachedCheckpoint {
        stageId,
        checkpoint: todo!( ),
        knownLatestBlockReachableByStage: None
      });
    }

    Ok(( ))
  }
}