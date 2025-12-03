// External crates
extern crate alloc;
use alloc::vec;
use humidifi_cpi::ID;
use litesvm::LiteSVM;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey;
use solana_sdk::sysvar::SysvarSerialize;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    message::Message,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::Keypair,
    signer::{EncodableKey, Signer},
    sysvar::rent::Rent,
};
use spl_token::state::Account as TokenAccount;
use std::convert::TryInto;

pub const HUMIDIFI_SWAP_SELECTOR: u8 = 0x4;
const HUMIDIFI_IX_DATA_KEY_SEED: [u8; 32] = [
    58, 255, 47, 255, 226, 186, 235, 195, 123, 131, 245, 8, 11, 233, 132, 219, 225, 40, 79, 119,
    169, 121, 169, 58, 197, 1, 122, 9, 216, 164, 149, 97,
];
pub const HUMIDIFI_IX_DATA_KEY: u64 = u64::from_le_bytes([
    HUMIDIFI_IX_DATA_KEY_SEED[0],
    HUMIDIFI_IX_DATA_KEY_SEED[1],
    HUMIDIFI_IX_DATA_KEY_SEED[2],
    HUMIDIFI_IX_DATA_KEY_SEED[3],
    HUMIDIFI_IX_DATA_KEY_SEED[4],
    HUMIDIFI_IX_DATA_KEY_SEED[5],
    HUMIDIFI_IX_DATA_KEY_SEED[6],
    HUMIDIFI_IX_DATA_KEY_SEED[7],
]);

const POS_INC: u64 = 0x0001_0001_0001_0001;
pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);
pub const PAYER: Pubkey = pubkey!("5BvrQfDzwjFFjpaAys2KA1a7GuuhLXKJoCWykhsoyHet");
pub const HUMIDIFI_PROGRAM_ID: Pubkey = pubkey!("9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp");
pub const HUMIDIFI_PROGRAM_ID_ARR: [u8; 32] = [
    122, 253, 116, 43, 39, 247, 89, 233, 198, 112, 112, 60, 211, 157, 129, 122, 160, 147, 10, 206,
    59, 82, 210, 109, 84, 160, 84, 221, 35, 135, 187, 211,
];
#[test]
fn test_process_humidifi_swap() {
    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com");
    let mut litesvm = LiteSVM::new();
    let kp = Keypair::new();
    litesvm.airdrop(&kp.pubkey(), 1000_000_000u64);
    litesvm
        .add_program_from_file(humidifi_cpi::ID, "./target/deploy/humidifi_cpi.so")
        .unwrap();
    litesvm
        .add_program_from_file(HUMIDIFI_PROGRAM_ID, "tests/humidifi.so")
        .unwrap();

    let wsol_mint = pubkey!("So11111111111111111111111111111111111111112");
    let usdc_mint = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    let pool = pubkey!("FksffEqnBRixYGR791Qw2MgdU7zNCpHVFYBL4Fa4qVuH");
    let pool_base_ata = pubkey!("C3FzbX9n1YD2dow2dCmEv5uNyyf22Gb3TLAEqGBhw5fY");
    let pool_quote_ata = pubkey!("3RWFAQBRkNGq7CMGcTLK3kXDgFTe9jgMeFYqk8nHwcWh");
    let clock_program = pubkey!("SysvarC1ock11111111111111111111111111111111");
    let token_program = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    let sysvar_ixs = pubkey!("Sysvar1nstructions1111111111111111111111111");
    let my_wsol_ata =
        spl_associated_token_account::get_associated_token_address(&kp.pubkey(), &wsol_mint);
    let my_usdc_ata =
        spl_associated_token_account::get_associated_token_address(&kp.pubkey(), &usdc_mint);
    litesvm
        .set_account(
            my_wsol_ata,
            get_dummy_token_account(&litesvm, kp.pubkey(), wsol_mint, token_program),
        )
        .unwrap();
    litesvm
        .set_account(
            my_usdc_ata,
            get_dummy_token_account(&litesvm, kp.pubkey(), usdc_mint, token_program),
        )
        .unwrap();
    litesvm.warp_to_slot(rpc.get_slot().unwrap());
    hydrate_svm(
        &mut litesvm,
        fetch_mainnet_accounts(
            vec![
                pool,
                pool_base_ata,
                pool_quote_ata,
                clock_program,
                token_program,
                wsol_mint,
                usdc_mint,
            ],
            &rpc,
        ),
    );
    litesvm.set_account(
        my_wsol_ata,
        get_dummy_token_account(&litesvm, kp.pubkey(), wsol_mint, token_program),
    );
    litesvm.set_account(
        my_usdc_ata,
        get_dummy_token_account(&litesvm, kp.pubkey(), usdc_mint, token_program),
    );

    let ix_accounts = vec![
        AccountMeta::new(kp.pubkey(), true),
        AccountMeta::new(pool, false),
        AccountMeta::new(pool_base_ata, false),
        AccountMeta::new(pool_quote_ata, false),
        AccountMeta::new(my_wsol_ata, false),
        AccountMeta::new(my_usdc_ata, false),
        AccountMeta::new_readonly(clock_program, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new_readonly(sysvar_ixs, false),
        AccountMeta::new(HUMIDIFI_PROGRAM_ID, false),
    ];
    // Create the instruction data
    let ix_data = 1000_000_000u64;
    // Ix discriminator = 0
    let mut ser_ix_data = vec![0];
    // Serialize the instruction data
    ser_ix_data.extend_from_slice(&ix_data.to_le_bytes());

    // Create instruction

    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);
    let message = Message::new(&[instruction], Some(&kp.pubkey()));
    let tx = solana_sdk::transaction::Transaction::new(
        &[kp.insecure_clone()],
        message,
        litesvm.latest_blockhash(),
    );
    // Execute transaction
    let result = litesvm.send_transaction(tx);
    if result.is_ok() {
        println!("{:#?}", result.unwrap().logs);
    } else {
        println!("{:#?}", result);
    }
}

pub fn get_dummy_token_account(
    svm: &LiteSVM,
    owner: Pubkey,
    mint: Pubkey,
    token_program: Pubkey,
) -> Account {
    let token_account = TokenAccount {
        mint,
        owner,
        amount: 10_000_000_000_000,
        delegate: None.into(),
        state: spl_token::state::AccountState::Initialized,
        is_native: None.into(),
        delegated_amount: 0,
        close_authority: None.into(),
    };

    let mut data = vec![0; TokenAccount::LEN];
    TokenAccount::pack(token_account, &mut data).unwrap();

    Account {
        lamports: svm.minimum_balance_for_rent_exemption(TokenAccount::LEN),
        data,
        owner: token_program,
        executable: false,
        rent_epoch: 0,
    }
}
pub fn hydrate_svm(litesvm: &mut LiteSVM, accounts: Vec<(Pubkey, Account)>) {
    for (addr, acc) in accounts {
        litesvm.set_account(addr, acc).unwrap();
    }
}

pub fn fetch_mainnet_accounts(
    addresses: Vec<Pubkey>,
    rpc_client: &RpcClient,
) -> Vec<(Pubkey, Account)> {
    let mut accounts: Vec<(Pubkey, Account)> = Vec::new();
    let base_accounts = rpc_client.get_multiple_accounts(&addresses).unwrap();

    for (i, acc_opt) in base_accounts.into_iter().enumerate() {
        if let Some(acc) = acc_opt {
            accounts.push((addresses[i], acc));
        }
    }
    accounts
}
