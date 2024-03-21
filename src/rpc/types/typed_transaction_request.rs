use ethers_core::types::{Address, Bytes, U256};
use super::transaction_request::AccessListItem;

pub enum TypedTransactionRequest {
  Legacy(LegacyTransactionRequest),
  EIP2930Based(EIP2930BasedTransactionRequest),
  EIP1559Based(EIP1559BasedTransactionRequest)
}

pub struct LegacyTransactionRequest {
  pub action: TransactionAction,
  pub senderAccountNonce: U256,
  pub value: U256,
  pub data: Bytes,
  pub gasLimit: U256,

  pub gasPrice: U256,

  pub chainId: Option<u64>
}

pub struct EIP2930BasedTransactionRequest {
  pub action: TransactionAction,
  pub senderAccountNonce: U256,
  pub value: U256,
  pub data: Bytes,
  pub gasLimit: U256,

  pub gasPrice: U256,

  pub accessList: Vec<AccessListItem>,

  pub chainId: u64
}

pub struct EIP1559BasedTransactionRequest {
  pub action: TransactionAction,
  pub senderAccountNonce: U256,
  pub value: U256,
  pub data: Bytes,
  pub gasLimit: U256,

  pub accessList: Vec<AccessListItem>,

  pub maxPriorityFeePerGas: U256,
  pub maxFeePerGas: U256,

  pub chainId: u64
}

pub enum TransactionAction {
  CallsAddress(Address),
  CreatesContract
}