#![allow(unexpected_cfgs)]
use crate::utils::*;
use arrayref::array_ref;
use arrayvec::ArrayVec;
use pinocchio::{
    account_info::AccountInfo,
    cpi::slice_invoke,
    default_panic_handler,
    instruction::{AccountMeta, Instruction},
    msg, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::clock::Clock,
    sysvars::{clock, instructions::IntrospectedAccountMeta, Sysvar},
    ProgramResult,
};
pub const HUMIDIFI_PROGRAM_ID: [u8; 32] = [
    122, 253, 116, 43, 39, 247, 89, 233, 198, 112, 112, 60, 211, 157, 129, 122, 160, 147, 10, 206,
    59, 82, 210, 109, 84, 160, 84, 221, 35, 135, 187, 211,
];
/*
accounts provided
program id
swap_auth
source ata
dest_ata
pool
pool base ata
pool quote ata
clock
token program
sysvar ixs
humidifi_param
*/
pub fn process_humidifi_swap(accounts: &[AccountInfo], amount_in: u64) -> ProgramResult {
    let humidifi_param_data: &[u8; 32] = &accounts[10].key();
    let swap_id = u64::from_le_bytes(
        humidifi_param_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    );

    let swap_params: SwapParams = SwapParams {
        swap_id: swap_id,
        amount_in,
        is_base_to_quote: false as u8, // only swap in pool direction for now
        padding: [0; 7],
    };

    let mut data: ArrayVec<u8, 25> = ArrayVec::new();
    let bytes: &[u8] = bytemuck::bytes_of(&swap_params);
    // To get owned Vec<u8>
    //let vec_bytes = bytemuck::bytes_of(&swap_params).to_vec();
    data.try_extend_from_slice(bytes);
    data.try_extend_from_slice(&[HUMIDIFI_SWAP_SELECTOR]);

    spin_instruction_data(&mut data);
    let account_infos = [
        &accounts[1],
        &accounts[4],
        &accounts[5],
        &accounts[6],
        &accounts[2],
        &accounts[3],
        &accounts[7],
        &accounts[8],
        &accounts[9],
    ];

    let accounts = [
        AccountMeta::readonly(&accounts[1].key()), //swap auth
        AccountMeta::writable(&accounts[4].key()), //pool
        AccountMeta::writable(&accounts[5].key()), //pool base ata
        AccountMeta::writable(&accounts[6].key()), //pool quote ata
        AccountMeta::writable(&accounts[2].key()), //base ata
        AccountMeta::writable(&accounts[3].key()), //quote ata
        AccountMeta::readonly(&accounts[7].key()), //clock
        AccountMeta::readonly(&accounts[8].key()), //token program
        AccountMeta::readonly(&accounts[9].key()), //sysvar ix
    ];
    let ix = Instruction {
        program_id: &HUMIDIFI_PROGRAM_ID,
        accounts: &accounts,
        data: &data,
    };
    slice_invoke(&ix, &account_infos)
}

program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
nostd_panic_handler!();
#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //data just contains disc and amount
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let array: [u8; 8] = instruction_data
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let amount_in: u64 = u64::from_le_bytes(array); // or from_be_bytes if big-endian
    process_humidifi_swap(accounts, amount_in)
}
