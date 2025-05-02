use alloy_primitives::U256;
use ethers::types::Transaction;
use ethers::utils::rlp;
use hex::FromHex;
use revm::primitives::alloy_primitives::{Address, Log};
use revm::primitives::TxKind;

#[derive(Debug)]
pub struct EvalTxResult {
    pub output: Vec<u8>,
    pub logs: Vec<Log>,
    pub gas_used: u64,
    pub status: bool,
    pub deployed_contract: Option<String>
}

#[derive(Debug)]
pub struct LoadEvmConfig {
    pub gas_limit: u64,
    pub gas_price: U256
}

impl Default for LoadEvmConfig  {
    fn default() -> Self {
        Self { gas_limit: 1_000_000_000, gas_price: U256::from(42) }
    }
}

impl LoadEvmConfig {
    pub fn custom(gas_limit: Option<u64>, gas_price: Option<u64>) -> Self {
        let gas_limit = gas_limit.unwrap_or(1_000_000_000);
        let gas_price = gas_price.unwrap_or(42);
        
        Self {
            gas_limit,
            gas_price: U256::from(gas_price)
        }
    }
}
pub fn recover_signer(raw_tx_hex: &str) -> Address {
    let raw_tx_bytes: Vec<u8> = Vec::from_hex(raw_tx_hex).unwrap();
    let tx: Transaction = rlp::decode(&raw_tx_bytes).unwrap();
    let signer = tx.recover_from().unwrap();
    Address::from_slice(signer.as_bytes())
}

pub fn get_tx_object(raw_tx_hex: &str) -> Transaction {
    let raw_tx_bytes = Vec::from_hex(raw_tx_hex).unwrap();
    rlp::decode(&raw_tx_bytes).unwrap()
}

pub fn get_tx_kind(tx: Transaction) -> TxKind {
    if tx.to.is_none() {
        TxKind::Create
    } else {
        let target = Address::from_slice(tx.to.unwrap().as_bytes());
        TxKind::Call(target)
    }
}

pub fn is_risc_v(calldata: &str) -> (bool, usize) {
    if let Ok(tx_bytes) = hex::decode(calldata) {
        // find the ELF header in the tx data
        let mut elf_start = 0;
        for i in 0..tx_bytes.len().saturating_sub(4) {
            if tx_bytes[i] == 0xff && i + 1 < tx_bytes.len() && 
               tx_bytes[i+1] == 0x7f && tx_bytes[i+2] == 0x45 && 
               tx_bytes[i+3] == 0x4c && tx_bytes[i+4] == 0x46 {
                elf_start = i;
                break;
            }
        }
        (elf_start > 0, elf_start)
    } else {
        (false, 0)
    }
}