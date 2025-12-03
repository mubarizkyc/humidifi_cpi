#![allow(unexpected_cfgs)]
use crate::utils::*;
use arrayref::array_ref;
use arrayvec::ArrayVec;
use pinocchio::{
    account_info::AccountInfo,
    cpi::slice_invoke,
    default_panic_handler,
    instruction::{AccountMeta, Instruction},
    no_allocator, program_entrypoint,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::clock::Clock,
    sysvars::{clock, instructions::IntrospectedAccountMeta, Sysvar},
    ProgramResult,
};
use pinocchio_token::state::TokenAccount;
use solana_msg::msg;
pub const HUMIDIFI_PROGRAM_ID: [u8; 32] = [
    122, 253, 116, 43, 39, 247, 89, 233, 198, 112, 112, 60, 211, 157, 129, 122, 160, 147, 10, 206,
    59, 82, 210, 109, 84, 160, 84, 221, 35, 135, 187, 211,
];

pub fn process_humidifi_swap(accounts: &[AccountInfo], amount_in: u64) -> ProgramResult {
    //sol to usdc;
    let balance_init = TokenAccount::from_account_info(&accounts[5])
        .unwrap()
        .amount();
    //pool accounts
    //base ata
    let sol_res = TokenAccount::from_account_info(&accounts[2])
        .unwrap()
        .amount() as u128;
    //  quote ata
    let usdc_res = TokenAccount::from_account_info(&accounts[3])
        .unwrap()
        .amount() as u128;
    msg!("Amount in: {}", amount_in);

    let fee_bps = 0u128; // 0.5%
    let dx_adj = (amount_in as u128) * (10_000 - fee_bps) / 10_000;

    // small trade approximation: linear
    let amount_out_est = dx_adj
        .checked_mul(usdc_res)
        .unwrap()
        .checked_div(sol_res)
        .unwrap();
    /*
    let amount_out_est = (amount_in as u128)
        .checked_mul(usdc_res)
        .unwrap()
        .checked_div(sol_res)
        .unwrap();
    */

    msg!("Amount out (estimated): {}", amount_out_est);
    let swap_params: SwapParams = SwapParams {
        swap_id: 0,
        amount_in,
        is_base_to_quote: false as u8, // only swap in pool direction for now
        padding: [0; 7],
    };

    let mut data: ArrayVec<u8, 25> = ArrayVec::new();

    let bytes: &[u8] = bytemuck::bytes_of(&swap_params);

    data.try_extend_from_slice(bytes);
    data.try_extend_from_slice(&[HUMIDIFI_SWAP_SELECTOR]);

    obfuscate_instruction_data(&mut data);

    let account_infos = [
        &accounts[0],
        &accounts[1],
        &accounts[2],
        &accounts[3],
        &accounts[4],
        &accounts[5],
        &accounts[6],
        &accounts[7],
        &accounts[8],
    ];

    let swap_accounts = [
        AccountMeta::new(&accounts[0].key(), true, true), //swap auth
        AccountMeta::writable(&accounts[1].key()),        //pool
        AccountMeta::writable(&accounts[2].key()),        //pool base ata
        AccountMeta::writable(&accounts[3].key()),        //pool quote ata
        AccountMeta::writable(&accounts[4].key()),        //base ata
        AccountMeta::writable(&accounts[5].key()),        //quote ata
        AccountMeta::readonly(&accounts[6].key()),        //clock
        AccountMeta::readonly(&accounts[7].key()),        //token program
        AccountMeta::readonly(&accounts[8].key()),        //sysvar ix
    ];
    let ix = Instruction {
        program_id: &HUMIDIFI_PROGRAM_ID,
        accounts: &swap_accounts,
        data: &data,
    };
    slice_invoke(&ix, &account_infos);

    let balance_final = TokenAccount::from_account_info(&accounts[5])?.amount();
    let amount_out_actual = balance_final.saturating_sub(balance_init);

    msg!("Amount out (actual): {}", amount_out_actual);
    msg!(
        "Deviation: {}",
        (amount_out_actual as i128) - (amount_out_est as i128)
    );

    Ok(())
}

program_entrypoint!(process_instruction);
//Do not allocate memory.
//no_allocator!();
// Use the no_std panic handler.
//nostd_panic_handler!();
#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //data just contains disc and amount
    let (_, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let array: [u8; 8] = instruction_data
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let amount_in: u64 = u64::from_le_bytes(array); // or from_be_bytes if big-endian
    process_humidifi_swap(accounts, amount_in)
}
/*
 let slot = Clock::get()?.slot;
    let pda = pinocchio::pubkey::find_program_address(
        &[
            accounts[1].key().as_ref(),
            accounts[2].key().as_ref(),
            slot.to_le_bytes().as_ref(),
        ],
        &HUMIDIFI_PROGRAM_ID,
    )
    .0;
*/
