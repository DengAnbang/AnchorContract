use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::instructions::CreateMetadataAccountV3Builder;
use mpl_token_metadata::types::DataV2;
use sha2::Digest;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use solana_sdk::commitment_config::CommitmentConfig;

fn create_metadata(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint: solana_program::pubkey::Pubkey,
    name: String,
    symbol: String,
    uri: String,
) -> Result<(), Box<dyn std::error::Error>> {

    // 创建 Metadata PDA
    let metadata: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint.as_ref(),
        ],
        &solana_program::pubkey::Pubkey::try_from(mpl_token_metadata::ID.as_ref()).unwrap(),
    ).0;

    // 创建元数据
    let data = DataV2 {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        seller_fee_basis_points: 500, // 5%
        creators: None,
        collection: None,
        uses: None,
    };

    // 使用 CreateV1Builder 构建指令
    let create_metadata_ix = CreateMetadataAccountV3Builder::new()
        .metadata(metadata)
        .mint(mint)
        .mint_authority(payer.pubkey())
        .is_mutable(true)
        .payer(payer.pubkey())
        .update_authority(payer.pubkey(), true)
        .data(data).instruction();

    // 构建并发送交易
    let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[create_metadata_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction successful: {}", signature);

    Ok(())
}