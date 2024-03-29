#![allow(non_snake_case)]

/*
  To follow and verify current data in the network, the Ethereum client needs to sync with the
  latest network state. This is done by downloading data from peers, cryptographically verifying
  their integrity, and building a local blockchain database.

  Synchronization modes represent different approaches to this process with various trade-offs.
  Clients also vary in their implementation of sync algorithms.

  EXECUTION LAYER SYNC MODES :

  1. Full archive sync : Full sync downloads all blocks (including headers, transactions, and
  receipts) and generates the state of the blockchain incrementally by executing every block from
  genesis. This, minimizes trust and offers the highest security by verifying every transaction. But
  also, with an increasing number of transactions, it can take days to weeks to process all
  transactions.

  2. Full snap sync : Snap sync verifies the chain block-by-block, just like a full archive sync -
  however, instead of starting at the genesis block, it starts at a more recent 'trusted' checkpoint
  that is known to be part of the true blockchain. The node saves periodic checkpoints while
  deleting data older than a certain age. Those snapshots are used to regenerate state data when it
  is needed, rather than having to store it all forever.

  3. Light sync (only takes a few minutes !) : Light client mode downloads all block headers, block
  data, and verifies some randomly. Only tip of the chain from the trusted checkpoint, is then synced.
  A light node gets only the latest state while relying on trust in developers and consensus
  mechanism.

  NOTE : Light sync does not yet work with proof-of-stake Ethereum - new versions of light sync
  should ship soon!

  Refernce : https://ethereum.org/en/developers/docs/nodes-and-clients/.
*/

pub mod stage;
pub mod stage_id;

pub mod stage_checkpoint;

pub mod pipeline;
mod pipeline_state;
pub mod pipeline_control_flow;
pub mod pipeline_error;
pub mod pipeline_event;

mod utils;

pub mod metrics;