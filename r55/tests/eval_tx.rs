use alloy_primitives::Address;
use r55::{
    exec::eval_tx,
    test_utils::{
        add_balance_to_db, initialize_logger, ALICE, BOB,
        CAROL,
    },
};
use revm::InMemoryDB;


#[test]
/// deploy and mint RISCV ERC20 tokens
fn test_eval_erc20() {
    std::env::set_var("RUST_LOG", "debug,revm=debug,r55=trace");
    initialize_logger();
    
    let mut db = InMemoryDB::default();
    let deployer = Address::from_slice(&hex::decode("b76FaBf56a6A9872efeA4EF848605D32eAfF13cE").unwrap());

    // Fund user accounts with some gas tokens
    for user in [ALICE, BOB, CAROL, deployer] {
        add_balance_to_db(&mut db, user, 1e18 as u64);
    }


    let raw_tx_signed = std::fs::read_to_string("signed-erc20-bytecode.txt").unwrap();
    let bytecode_calldata = raw_tx_signed.trim();
    let calldata = bytecode_calldata.trim_start_matches("0x");
    
    println!("using calldata with prefix: {}", &calldata[..std::cmp::min(100, calldata.len())]);
    
    // Check if this is a RISCV transaction
    if let Ok(tx_bytes) = hex::decode(calldata) {
        for i in 0..tx_bytes.len().saturating_sub(4) {
            if tx_bytes[i] == 0xff && tx_bytes[i+1] == 0x7f && 
               tx_bytes[i+2] == 0x45 && tx_bytes[i+3] == 0x4c && 
               tx_bytes[i+4] == 0x46 {
                println!("found RISCV ELF header at position {}", i);
                break;
            }
        }
    }

    // deploy the RISCV ERC20 bytecode
    match eval_tx(&mut db, calldata) {
        Ok(res) => {
            println!("deploy tx successful: {:?}", res);
            
        },
        Err(e) => {
            println!("deploy tx failed: {:?}", e);
        }
    }

    // now we mint tokens (calldata generated using tests/erc20.rs @ test_erc20_mint()) then signed using deployer address
    let file_content = std::fs::read_to_string("signed-erc20-mint.txt").unwrap();
    let mint_calldata = file_content.trim();
    let mint_calldata = mint_calldata.trim_start_matches("0x");
    println!("mint_calldata: {:?}", mint_calldata);
    match eval_tx(&mut db, mint_calldata) {
        Ok(res) => {
            println!("mint tx successful: {:?}", res);
        },
        Err(e) => {
            println!("mint tx failed: {:?}", e);
        }
    }
}
