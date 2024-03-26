use ethers_core::types::BlockNumber;
use tokio::sync::mpsc::Sender;
use super::pipeline_event::PipelineEvent;

pub(in crate::sync) struct PipelineState {
  pub(in crate::sync) eventsSender: Option<Sender<PipelineEvent>>,
  pub(in crate::sync) stopSyncAfterReachingBlock: Option<BlockNumber>,

  // Block-number of the oldest block that this pipeline dealt with during execution.
  pub(in crate::sync) oldestBlockReached: Option<BlockNumber>,

  // Block-number of the most recent block that this pipeline dealt with during execution.
  pub(in crate::sync) mostRecentBlockReached: Option<BlockNumber>,

  // Whether or not the previous stage reached the chain tip.
  pub(in crate::sync) reachedChainTip: bool
}