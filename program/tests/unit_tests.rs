// External crates
extern crate alloc;
use alloc::vec;
use solana_sdk::sysvar::Sysvar;
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
// Mollusk
use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::sysvar::Sysvars;
use mollusk_svm::{program, sysvar, Mollusk};

// Solana SDK
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::SystemInstruction,
    system_program,
};

// Local crate(s)
use humidifi_cpi::ID;

// Std
use std::convert::TryInto;
const POS_INC: u64 = 0x0001_0001_0001_0001;
pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);
pub const PAYER: Pubkey = pubkey!("5BvrQfDzwjFFjpaAys2KA1a7GuuhLXKJoCWykhsoyHet");
pub const HUMIDIFI_PROGRAM_ID: Pubkey = pubkey!("9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp");
pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/exp");
    mollusk
}

#[test]
fn test_process_humidifi_swap() {
    let mut mollusk = mollusk();
    mollusk_svm_programs_token::token::add_program(&mut mollusk);

    mollusk.add_program(
        &HUMIDIFI_PROGRAM_ID,
        "/home/mubariz/Documents/SolDev/humidifi_cpi/program/tests/humidifi",
        &mollusk_svm::program::loader_keys::LOADER_V3,
    );

    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com ");
    let me = PAYER;
    let pool = pubkey!("FksffEqnBRixYGR791Qw2MgdU7zNCpHVFYBL4Fa4qVuH");
    let pool_base_ata = pubkey!("C3FzbX9n1YD2dow2dCmEv5uNyyf22Gb3TLAEqGBhw5fY");
    let pool_quote_ata = pubkey!("3RWFAQBRkNGq7CMGcTLK3kXDgFTe9jgMeFYqk8nHwcWh");
    let my_wsol_ata = pubkey!("689gZnbWXCGDcTwqknp9CtRZGgrHxFmhQKBCFBcJWeJY");
    let my_usdc_ata = pubkey!("GSBto5i58DWh8jimTLqhq5eC1KUZKX5grNYFeYyGT8K");
    let clock_program = pubkey!("SysvarC1ock11111111111111111111111111111111");
    let token_program = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    let sysvar_ixs = pubkey!("Sysvar1nstructions1111111111111111111111111");
    let humidifi_param_address = pubkey!("FrygL227VFGsKFMo8RUHzKoGRtg9LAqrucPwPi6dy74T"); //TODO:Call jerry

    let accounts_addresses = [
        HUMIDIFI_PROGRAM_ID,
        me,
        pool,
        pool_base_ata,
        pool_quote_ata,
        my_wsol_ata,
        my_usdc_ata,
        clock_program,
        token_program,
    ];
    let mainnet_accounts = rpc.get_multiple_accounts(&accounts_addresses).unwrap();

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(HUMIDIFI_PROGRAM_ID, false),
        AccountMeta::new(PAYER, true),
        AccountMeta::new(pool, false),
        AccountMeta::new(pool_base_ata, false),
        AccountMeta::new(pool_quote_ata, false),
        AccountMeta::new(my_wsol_ata, false),
        AccountMeta::new(my_usdc_ata, false),
        AccountMeta::new_readonly(clock_program, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new_readonly(sysvar_ixs, false),
        AccountMeta::new_readonly(humidifi_param_address, false),
    ];

    // Create the instruction data
    let ix_data = 100000u64;
    // Ix discriminator = 0
    let mut ser_ix_data = vec![0];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(&ix_data.to_le_bytes());

    // Create instruction
    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

    let sysvar_account = Account::new(0, 100, &solana_sdk::sysvar::id());
    let humidifi_param_account = Account::new(0, 0, &solana_sdk::system_program::id());

    let tx_accounts = &vec![
        (
            HUMIDIFI_PROGRAM_ID,
            mainnet_accounts[0].clone().expect("HIMIDIFI   NOT found"),
        ),
        (
            PAYER,
            mainnet_accounts[1]
                .clone()
                .expect("PAYER account not found"),
        ),
        (
            pool,
            mainnet_accounts[2].clone().expect("Pool account not found"),
        ),
        (
            pool_base_ata,
            mainnet_accounts[3]
                .clone()
                .expect("Pool base ATA not found"),
        ),
        (
            pool_quote_ata,
            mainnet_accounts[4]
                .clone()
                .expect("Pool quote ATA not found"),
        ),
        (
            my_wsol_ata,
            mainnet_accounts[5].clone().expect("My WSOL ATA not found"),
        ),
        (
            my_usdc_ata,
            mainnet_accounts[6].clone().expect("My USDC ATA not found"),
        ),
        (
            clock_program,
            mainnet_accounts[7].clone().expect("Clock sysvar not found"),
        ),
        (
            token_program,
            mainnet_accounts[8]
                .clone()
                .expect("Token program not found"),
        ),
        (sysvar_ixs, sysvar_account),
        (humidifi_param_address, humidifi_param_account),
    ];

    let init_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(init_res.program_result == ProgramResult::Success);
}

fn deobfuscate(data: &mut [u8]) {
    let mut pos_mask: u64 = 0;
    let mut i = 0usize;
    while i < data.len() {
        let end = core::cmp::min(i + 8, data.len());
        let mut tmp = [0u8; 8];
        tmp[..end - i].copy_from_slice(&data[i..end]);
        let mut q = u64::from_le_bytes(tmp);
        // reverse: q ^= pos_mask; q ^= key
        q ^= pos_mask;
        q ^= HUMIDIFI_IX_DATA_KEY;
        let out = q.to_le_bytes();
        data[i..end].copy_from_slice(&out[..end - i]);
        pos_mask = pos_mask.wrapping_add(POS_INC);
        i += 8;
    }
}

fn parse_swap_params(data: &[u8]) -> Option<(u64, u64, u8)> {
    if data.len() < 17 {
        return None;
    }
    let swap_id = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let amount_in = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let flag = data[16];
    Some((swap_id, amount_in, flag))
}

pub fn get_rent_data() -> Vec<u8> {
    let rent = Rent::default();
    unsafe {
        core::slice::from_raw_parts(&rent as *const Rent as *const u8, Rent::size_of()).to_vec()
    }
}

use hex;
#[test]
fn test_rev_eng() {
    println!("test start");
    let hex_str = "3ab9c6592f2e4c92fba876e2e3baeac338ff2dffe0bae9c33d";
    let mut buf = hex::decode(hex_str).expect("Invalid hex string");

    deobfuscate(&mut buf);
    if let Some((swap_id, amount_in, flag)) = parse_swap_params(&buf) {
        println!(
            "swap_id={}, amount_in={}, flag={}",
            swap_id, amount_in, flag
        );
    } else {
        println!("buffer too short or invalid after deobfuscation");
    }
}
