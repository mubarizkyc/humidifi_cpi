// External crates
extern crate alloc;
use alloc::vec;
use humidifi_cpi::utils::{obfuscate_instruction_data, SwapParams};
use litesvm::LiteSVM;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKey;
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

// Solana SDK
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
    rent::Rent,
};

// Local crate(s)
use humidifi_cpi::ID;

// Std
use std::convert::TryInto;
const POS_INC: u64 = 0x0001_0001_0001_0001;
pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);
pub const PAYER: Pubkey = pubkey!("5BvrQfDzwjFFjpaAys2KA1a7GuuhLXKJoCWykhsoyHet");
pub const HUMIDIFI_PROGRAM_ID: Pubkey = pubkey!("9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp");
pub const HUMIDIFI_PROGRAM_ID_ARR: [u8; 32] = [
    122, 253, 116, 43, 39, 247, 89, 233, 198, 112, 112, 60, 211, 157, 129, 122, 160, 147, 10, 206,
    59, 82, 210, 109, 84, 160, 84, 221, 35, 135, 187, 211,
];

#[tokio::test(flavor = "multi_thread")]
async fn test_process_humidifi_swap() {
    let mut litesvm = LiteSVM::new();
    litesvm
        .add_program_from_file(humidifi_cpi::ID, "./target/deploy/humidifi_cpi.so")
        .unwrap();
    litesvm
        .add_program_from_file(
            HUMIDIFI_PROGRAM_ID,
            "/home/mubariz/Documents/SolDev/humidifi_cpi/program/tests/humidifi.so",
        )
        .unwrap();
    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com");
    let wsol_mint = pubkey!("So11111111111111111111111111111111111111112");
    let usdc_mint = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    let me = PAYER;
    let pool = pubkey!("FksffEqnBRixYGR791Qw2MgdU7zNCpHVFYBL4Fa4qVuH");
    let pool_base_ata = pubkey!("C3FzbX9n1YD2dow2dCmEv5uNyyf22Gb3TLAEqGBhw5fY");
    let pool_quote_ata = pubkey!("3RWFAQBRkNGq7CMGcTLK3kXDgFTe9jgMeFYqk8nHwcWh");
    let my_wsol_ata = pubkey!("689gZnbWXCGDcTwqknp9CtRZGgrHxFmhQKBCFBcJWeJY");
    let my_usdc_ata = pubkey!("GSBto5i58DWh8jimTLqhq5eC1KUZKX5grNYFeYyGT8K");
    let clock_program = pubkey!("SysvarC1ock11111111111111111111111111111111");
    let token_program = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    let sysvar_ixs = pubkey!("Sysvar1nstructions1111111111111111111111111");
    while (true) {
        litesvm.warp_to_slot(rpc.get_slot().unwrap());
        hydrate_svm(
            &mut litesvm,
            fetch_mainnet_accounts(
                vec![
                    me,
                    pool,
                    pool_base_ata,
                    pool_quote_ata,
                    clock_program,
                    token_program,
                    wsol_mint,
                    usdc_mint,
                ],
                &rpc,
            )
            .await,
        );
        litesvm.set_account(
            my_wsol_ata,
            get_dummy_token_account(&litesvm, me, wsol_mint, token_program),
        );
        litesvm.set_account(
            my_usdc_ata,
            get_dummy_token_account(&litesvm, me, usdc_mint, token_program),
        );

        let ix_accounts = vec![
            AccountMeta::new(PAYER, true),
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
        let kp = Keypair::read_from_file("/home/mubariz/wallnuts/mainnet-keypair.json").unwrap();
        let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);
        let message =
            Message::try_compile(&me, &[instruction], &[], litesvm.latest_blockhash()).unwrap();
        let tx = VersionedTransaction::try_new(VersionedMessage::V0(message), &[kp]).unwrap();

        // Execute transaction
        let result = litesvm.send_transaction(tx);
        if result.is_ok() {
            println!("{:#?}", result.unwrap().logs);
        } else {
            println!("{:#?}", result);
        }
    }
}
use solana_sdk::program_pack::Pack;
use spl_token::state::Account as TokenAccount;
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

pub async fn fetch_mainnet_accounts(
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
use solana_sdk::sysvar::SysvarSerialize;
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

fn hex_of(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}
use solana_sdk::message::{v0::Message, VersionedMessage};
use solana_sdk::transaction::VersionedTransaction;
pub fn obfuscate_instruction_data_local(data: &mut [u8]) {
    let mut pos_mask = 0_u64;

    // Process full 8-byte qwords
    for chunk in data.chunks_exact_mut(8) {
        // convert 8 bytes -> u64 (little-endian)
        let mut v = u64::from_le_bytes(chunk.try_into().unwrap());
        v ^= HUMIDIFI_IX_DATA_KEY;
        v ^= pos_mask;
        pos_mask = pos_mask.wrapping_add(0x0001_0001_0001_0001);
        // write back
        chunk.copy_from_slice(&v.to_le_bytes());
    }

    // Handle remainder (0..7 bytes)
    let remainder = data.chunks_exact_mut(8).into_remainder();
    if !remainder.is_empty() {
        // copy remainder into a u64, operate, then copy back only the used bytes
        let mut rem = 0_u64;
        // copy remainder bytes into the low bytes of rem
        for (i, &b) in remainder.iter().enumerate() {
            rem |= (b as u64) << (8 * i);
        }
        rem ^= HUMIDIFI_IX_DATA_KEY;
        rem ^= pos_mask;
        let rem_bytes = rem.to_le_bytes();
        for (i, out) in remainder.iter_mut().enumerate() {
            *out = rem_bytes[i];
        }
    }
}
/*
#[tokio::test(flavor = "multi_thread")]
async fn swap_humidifi() {
    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com");
    let kp = Keypair::read_from_file("/home/mubariz/wallnuts/mainnet-keypair.json").unwrap();
    let me = PAYER;
    let pool = pubkey!("FksffEqnBRixYGR791Qw2MgdU7zNCpHVFYBL4Fa4qVuH");
    let pool_base_ata = pubkey!("C3FzbX9n1YD2dow2dCmEv5uNyyf22Gb3TLAEqGBhw5fY");
    let pool_quote_ata = pubkey!("3RWFAQBRkNGq7CMGcTLK3kXDgFTe9jgMeFYqk8nHwcWh");
    let my_wsol_ata = pubkey!("689gZnbWXCGDcTwqknp9CtRZGgrHxFmhQKBCFBcJWeJY");
    let my_usdc_ata = pubkey!("GSBto5i58DWh8jimTLqhq5eC1KUZKX5grNYFeYyGT8K");
    let clock_program = pubkey!("SysvarC1ock11111111111111111111111111111111");
    let token_program = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    let sysvar_ixs = pubkey!("Sysvar1nstructions1111111111111111111111111");
    let slot = rpc.get_slot().unwrap();
    let pda = Pubkey::find_program_address(
        &[me.as_ref(), pool.as_ref(), slot.to_le_bytes().as_ref()],
        &HUMIDIFI_PROGRAM_ID,
    )
    .0;
    let swap_params: SwapParams = SwapParams {
        swap_id: 0u64,
        amount_in: 100_000_000u64,
        is_base_to_quote: false as u8, // only swap in pool direction for now
        padding: [0; 7],
    };
    let mut data: arrayvec::ArrayVec<u8, 25> = arrayvec::ArrayVec::new();

    let bytes: &[u8] = bytemuck::bytes_of(&swap_params);

    data.try_extend_from_slice(bytes);
    data.try_extend_from_slice(&[HUMIDIFI_SWAP_SELECTOR]);

    obfuscate_instruction_data_local(&mut data);

    let accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(pool, false),
        AccountMeta::new(pool_base_ata, false),
        AccountMeta::new(pool_quote_ata, false),
        AccountMeta::new(my_wsol_ata, false),
        AccountMeta::new(my_usdc_ata, false),
        AccountMeta::new_readonly(clock_program, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new_readonly(sysvar_ixs, false),
    ];
    let ix = Instruction {
        program_id: HUMIDIFI_PROGRAM_ID,
        data: data.to_vec(),
        accounts,
    };
    let bh = rpc.get_latest_blockhash().unwrap();

    let message = Message::try_compile(&PAYER, &[ix], &[], bh).unwrap();
    let tx = VersionedTransaction::try_new(VersionedMessage::V0(message), &[kp]).unwrap();
    let result = rpc.simulate_transaction(&tx).unwrap();
    println!("Result {:?}", result);
}
*/

/*
#[tokio::test]
async fn main() {
    let slot: u64 = 378935367;
    let auth = Pubkey::from_str_const("7dGrdJRYtsNR8UYxZ3TnifXGjGc9eRYLq9sELwYpuuUu");
    let caller_program = Pubkey::from_str_const("King7ki4SKMBPb3iupnQwTyjsq294jaXsgLmJo8cb7T");
    let pool = Pubkey::from_str_const("FksffEqnBRixYGR791Qw2MgdU7zNCpHVFYBL4Fa4qVuH");
    //current solt 378 755 428
    let hex_str = "5cf3f2645bcd1d4755fa4284e0baeac339ff2dffe0bae9c33d";

    let mut buf = hex::decode(hex_str).expect("Invalid hex string");
    deobfuscate(&mut buf);

    // print hex after deobfuscation
    println!("deobfuscated hex: {}", hex_of(&buf));

    if let Some((swap_id, amount_in, flag)) = parse_swap_params(&buf) {
        println!(
            "swap_id: {}, amount_in: {}, flag: {}",
            swap_id, amount_in, flag
        );
    } else {
        println!("parse failed: buffer too short or invalid");
    }
    //auth slot pool
    //auth pool slot
    //pool slot auth
    //slot auth pool
    let pda = Pubkey::find_program_address(
        &[auth.as_ref(), pool.as_ref(), slot.to_le_bytes().as_ref()],
        &HUMIDIFI_PROGRAM_ID,
    )
    .0
    .to_bytes();
    println!(
        "swap_id_derived: {}",
        u64::from_le_bytes(pda[0..8].try_into().unwrap())
    );
}
*/
//program BLzDYkUcxzBv6ne24N2YyPECoYTvDTKaukbbEemKASc4
//transfer auth DFDjLXZCkkQzDmZHr2CLrkX1pmgpUs18gEakJDxz5VYV
//85c2abb5da9cd4184a83559aeebaeac338ff2dffe0bae9c33d //slot (378752133)
//b4e7a2f20c4b36cc19579edbe7baeac339ff2dffe0bae9c33d//(slot)  (378763514)

//slot : 378935367
//pool : FksffEqnBRixYGR791Qw2MgdU7zNCpHVFYBL4Fa4qVuH
//program : King7ki4SKMBPb3iupnQwTyjsq294jaXsgLmJo8cb7T
//singer: 7dGrdJRYtsNR8UYxZ3TnifXGjGc9eRYLq9sELwYpuuUu
//ix_data: 5cf3f2645bcd1d4755fa4284e0baeac339ff2dffe0bae9c33d
