use std::io::Read;
use std::str::FromStr;
use std::time::Duration;

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey, system_program};
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::Signer
    ,
    transaction::Transaction,
};
use solana_sdk::signature::{keypair, Keypair};

use crate::state::Notes;

mod instruction;
mod state;

// const RPC_ADDR: &str = "https://api.devnet.solana.com";
const RPC_ADDR: &str = "http://localhost:8899";

fn main() {
    let client = RpcClient::new_with_commitment(RPC_ADDR.to_string(), CommitmentConfig::confirmed());
    let program_id = pubkey::Pubkey::from_str("CKyE2drXuaYcbBF8japHFSQTgEShYfGBtZKjJuN1nMT3").unwrap();
    let payer = keypair::Keypair::from_base58_string("63gS1D49STGGYkKH7SoVgtf628HjbqjqPnwYPAXLFYEjPqY6ENyR1SGUGRL3kXmLUp9Lw6Jr3oKo9vb3zXv2VNXZ");
    let (pda, _bump) = Pubkey::find_program_address(&[
        &payer.pubkey().as_ref(),
    ], &program_id);
    println!("pda: {}", pda.to_string());
    create(&client, &program_id, &payer, &pda);
    std::thread::sleep(Duration::from_secs(5));
    reading(&client, &pda);
    modification(&client, &program_id, &payer, &pda);
    std::thread::sleep(Duration::from_secs(5));
    reading(&client, &pda);
    std::thread::sleep(Duration::from_secs(5));
    delete(&client, &program_id, &payer, &pda);


    // let discriminator = calculate_discriminator("modification");
    // let data = borsh::to_vec(&Instruction::Create("大阿三大苏打".parse().unwrap())).unwrap();
    // // [24, 30, 200, 40, 5, 28, 7, 119, 18, 0, 0, 0, 229, 164, 167, 233, 152, 191, 228, 184, 137, 229, 164, 167, 232, 139, 143, 230, 137, 147]
    // let x = "大阿三大苏打".to_string().as_bytes();
    // println!("notes: {:?} ", "大阿三大苏打".to_string().as_bytes());
}

fn reading(client: &RpcClient, pda: &Pubkey) {
    let discriminator_len = 8;
    // 读取并解析账户数据
    let account_data = client.get_account_data(&pda).unwrap();
    let length = u32::from_le_bytes(account_data[0 + discriminator_len..4 + discriminator_len].try_into().unwrap()) as usize;
    let v = &account_data[0 + discriminator_len..length + 4 + discriminator_len];
    let greeting_account: Notes = Notes::try_from_slice(v).unwrap();
    println!("读取数据: {} ", greeting_account.notes);
}

fn delete(client: &RpcClient, program_id: &Pubkey, payer: &Keypair, pda: &Pubkey) {
    // 构建交易
    let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        *program_id,
        &calculate_discriminator("delete"),
        vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(*pda, false),
        ],
    );
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}

fn modification(client: &RpcClient, program_id: &Pubkey, payer: &Keypair, pda: &Pubkey) {
    // 构建交易
    let mut vec = calculate_discriminator("modification").to_vec();
    let mut data = borsh::to_vec(&Notes { notes: "修改后的修改后的".to_string() }).unwrap();
    println!("modification data: {:?} ", &vec);
    vec.append(&mut data);
    let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        *program_id,
        vec.as_slice(),
        vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(*pda, false),
            solana_sdk::instruction::AccountMeta::new(system_program::ID, false),
        ],
    );
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
    // 读取并解析账户数据
    let account_data = client.get_account_data(&pda).unwrap();
    println!("account_data {:?} ", &account_data[..]);
}

fn create(client: &RpcClient, program_id: &Pubkey, payer: &Keypair, pda: &Pubkey) {
    // 构建交易
    let mut vec = calculate_discriminator("create").to_vec();
    let mut data = borsh::to_vec(&Notes { notes: "创建的".to_string() }).unwrap();
    vec.append(&mut data);
    println!("create data: {:?} ", &vec);
    let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        *program_id,
        vec.as_slice(),
        vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(*pda, false),
            solana_sdk::instruction::AccountMeta::new(system_program::ID, false),
        ],
    );
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
    // 读取并解析账户数据
    let account_data = client.get_account_data(&pda).unwrap();
    println!("account_data {:?} ", &account_data[..]);
}


fn calculate_discriminator(account_type: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", "global", account_type);
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());

    let result = hasher.finalize();

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);

    discriminator
}
