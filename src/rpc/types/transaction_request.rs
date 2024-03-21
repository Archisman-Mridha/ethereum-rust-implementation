use alloy_primitives::{Address, Bytes, U256};
use ethers_core::types::H256;
use serde::{Deserialize, Serialize};
use super::typed_transaction_request::*;

#[derive(Serialize, Deserialize)]
pub struct TransactionRequest {
  pub from: Option<Address>,
  /*
    When you send a transaction and once it’s mined, your account increments a value called nonce
    by one. The nonce keeps track of how many transactions the sender has sent overtime.

    Why do we need account nonce ?

    1. It allows us to send transactions in order :
    Without the nonce, the order in which you send transactions might be ignored. Let’s say you send
    two transactions within 10 seconds. You might think that miners process them in order. But in a
    distributed system, there is no guarantee that they collect your first transaction first and the
    second one second. Yet, if you have the account nonce, miners need to process them in order. The
    second transaction with a nonce of 1 cannot be processed until the first transaction with a nonce
    of 0 is processed.

    2. It protects us from replay attacks :
    Without the nonce, a sender could double-spend by sending the same transaction twice. Let’s say
    Alice signs a transaction that sends some ether to Bob. Without the nonce, Bob could copy this
    signed transaction and keep propagating it until Alice loses all her money. Since the
    transaction is not unique without a nonce, Bob can duplicate the transaction. He doesn’t even
    have to sign it as he can find the signed transaction in the network already.

    When a transaction includes the nonce, every single transaction (with the same amount and the
    same recipient address) is unique.

    Reference : https://medium.com/coinmonks/the-account-nonce-in-ethereum-explained-c087bd4a3c29.
  */
  #[serde(rename = "nonce")]
  pub senderAccountNonce: Option<U256>, // (including this transaction)

  pub to: Option<Address>,

  pub value: Option<U256>, // (in wei)

  // Additional data sent using the transaction.
  pub data: Option<Bytes>,

  /*
    Gas :

    Gas refers to the unit that measures the amount of computational effort required to execute
    specific operations on the Ethereum network.

    Since each Ethereum transaction requires computational resources to execute, those resources
    have to be paid for. Payment for computation is made in the form of a gas fee. It's the amount
    of gas used to do some operation, multiplied by the cost per unit gas. The fee is paid
    regardless of whether a transaction succeeds or fails.

    Gas fees have to be paid in Ethereum's native currency, ether (ETH).

    You can set the amount of gas fee you are willing to pay when you submit a transaction. By
    offering a certain amount of gas fee, you are bidding for your transaction to be included in the
    next block. If you offer too little, validators are less likely to choose your transaction for
    inclusion, meaning your transaction may execute late or not at all. If you offer too much, you
    might waste some ETH. So, how can you tell how much to pay?

    The total gas fee you pay is divided into two components:

    1. Base Fee - is set by the protocol. You have to pay at least this amount for your transaction
    to be considered valid.

    2. Priority Fee - is a tip that you add to the base fee to make your transaction attractive to
    validators so that they choose it for inclusion in the next block. The 'correct' priority fee is
    determined by the network usage at the time you send your transaction - if there is a lot of
    demand then you might have to set your priority fee higher, but when there is less demand you
    can pay less.

    Reference : https://ethereum.org/en/developers/docs/gas/.
  */
  pub gasLimit: Option<U256>,

  /*
    Tranasaciton types :

    1. Legacy transactions (type 0x0) :

    They use the transaction format existing before typed transactions were introduced in EIP-2718.
    They contain the parameters - nonce, to, value, data, gasLimit, gasPrice (fee per unit gas the
    sender is willing to pay to the miner).

    2. EIP-2930 based tranasactions (type 0x1) :
    
    They contain, along with the legacy parameters, an 'accessList' parameter, which specifies an
    array of addresses and storage keys that the transaction plans to access.
    NOTE : They mustn't incorporate EIP-1559.

    3. EIP-1559 based transactions (type 0x2) :

    Transactions with type 0x2 are transactions introduced in EIP-1559, included in Ethereum's
    London fork. EIP-1559 addresses the network congestion and overpricing of transaction fees
    caused by the historical fee market, in which users send transactions specifying a gas price bid
    using the gasPrice parameter, and miners choose transactions with the highest bids.

    EIP-1559 transactions don’t specify gasPrice, and instead use an in-protocol, dynamically
    changing base fee per gas. At each block, the base fee per gas is adjusted to address network
    congestion as measured by a gas target.

    EIP-1559 transactions contain the accessList and legacy parameters (except for gasPrice). They
    also contain a maxPriorityFeePerGas parameter, which specifies the maximum fee the sender is
    willing to pay per gas above the base fee (the maximum priority fee per gas), and a maxFeePerGas
    parameter, which specifies the maximum total fee (base fee + priority fee) the sender is willing
    to pay per gas.

    Reference : https://docs.infura.io/api/networks/ethereum/concepts/transaction-types.
  */
  #[serde(rename = "type")]
  pub transactionType: Option<U256>,

  pub gasPrice: Option<U256>,

  pub accessList: Option<Vec<AccessListItem>>,

  pub maxPriorityFeePerGas: Option<U256>,
  pub maxFeePerGas: Option<U256>
}

impl TransactionRequest {
  // Converts TransactionRequest to TypedTransactionRequest.
  pub fn intoTypedTransactionRequest(self) -> Option<TypedTransactionRequest> {
    let action= match self.to {
      Some(to) => TransactionAction::CallsAddress(to),
      None => TransactionAction::CreatesContract
    };
    let senderAccountNonce= self.senderAccountNonce.unwrap_or(U256::ZERO);
    let value= self.value.unwrap_or(U256::ZERO);
    let data= self.data.unwrap_or_default( );
    let gasLimit= self.gasLimit.unwrap_or_default( );

    let TransactionRequest {
      gasPrice,

      accessList,

      maxPriorityFeePerGas,
      maxFeePerGas,
      ..
    }= self;

    match (gasPrice, accessList, maxFeePerGas) {

      // Legacy transaction.
      (Some(gasPrice), None, None) => {
        Some(TypedTransactionRequest::Legacy(LegacyTransactionRequest {
          action,
          senderAccountNonce,
          value,
          data,
          gasLimit,

          gasPrice,

          chainId: None
        }))
      },

      // EIP-2930 based transaction.
      (gasPrice, Some(accessList), None) => {
        Some(TypedTransactionRequest::EIP2930Based(EIP2930BasedTransactionRequest {
          action,
          senderAccountNonce,
          value,
          data,
          gasLimit,

          gasPrice: gasPrice.unwrap_or_default( ),

          accessList,

          chainId: 0
        }))
      },

      // EIP-1559 based transaction.
      (None, accessList, Some(maxFeePerGas)) => {
        Some(TypedTransactionRequest::EIP1559Based(EIP1559BasedTransactionRequest {
          action,
          senderAccountNonce,
          value,
          data,
          gasLimit,

          accessList: accessList.unwrap_or_default( ),

          maxPriorityFeePerGas: maxPriorityFeePerGas.unwrap_or(U256::ZERO),
          maxFeePerGas,

          chainId: 0
        }))
      },

      _ => None
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct AccessListItem {
  pub address: Address,
  pub storageKeys: Vec<H256>
}