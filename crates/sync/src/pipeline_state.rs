use ethers_core::types::BlockNumber;
use tokio::sync::mpsc::Sender;
use super::pipeline_event::PipelineEvent;

pub(crate) struct PipelineState {
  pub(crate) eventsSender: Option<Sender<PipelineEvent>>,
  pub(crate) stopSyncAfterReachingBlock: Option<BlockNumber>,

  // Block-number of the oldest block that this pipeline dealt with during execution.
  pub(crate) oldestBlockReached: Option<BlockNumber>,

  // Block-number of the most recent block that this pipeline dealt with during execution.
  pub(crate) mostRecentBlockReached: Option<BlockNumber>,

  // Whether or not the previous stage reached the chain tip.
  pub(crate) reachedChainTip: bool
}