## Types of Ethereum Nodes

Clients can run three different types of nodes: `light`, `full` and `archive`.

1. `Full node` :

> Full nodes do a block-by-block validation of the blockchain, including downloading and verifying the block body and state data for each block.

There are different classes of full node - some **start from the genesis block and verify every single block in the entire history** of the blockchain. Others **start their verification at a more recent block that they trust** to be valid (e.g. Geth's `snap sync`).

Regardless of where the verification starts, full nodes **only keep a local copy of relatively recent data** (typically the most recent 128 blocks), allowing older data to be deleted to save disk space. Older data can be regenerated (from `snapshots`) when it's needed.

2. `Archive node` :

> Archive nodes are full nodes that verify every block from genesis and never delete any of the downloaded data.

3. `Light node` :

> Instead of downloading every block, light nodes **only download block headers**. These headers contain summary information about the contents of the blocks. **Any other information the light node required gets requested from a full node**. The light node can then independently verify the data they receive against the state roots in the block headers.

Light nodes enable users to participate in the Ethereum network without the powerful hardware or high bandwidth required to run full nodes. Eventually, light nodes might run on mobile phones or embedded devices. The light nodes do not participate in consensus (i.e. they cannot be miners/validators), but they can access the Ethereum blockchain with the same functionality and security guarantees as a full node.