#![allow(unexpected_cfgs)]
use crate::utils::*;
use arrayvec::ArrayVec;
use pinocchio::{
    account_info::AccountInfo,
    cpi::slice_invoke,
    instruction::{AccountMeta, Instruction},
    program_entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

pub const HUMIDIFI_PROGRAM_ID: [u8; 32] = [
    122, 253, 116, 43, 39, 247, 89, 233, 198, 112, 112, 60, 211, 157, 129, 122, 160, 147, 10, 206,
    59, 82, 210, 109, 84, 160, 84, 221, 35, 135, 187, 211,
];
pub fn process_humidifi_swap(accounts: &[AccountInfo], amount_in: u64) -> ProgramResult {
    let swap_params: SwapParams = SwapParams {
        swap_id: 0,
        amount_in,
        is_base_to_quote: false as u8,
        padding: [0; 7],
    };
    let mut data: ArrayVec<u8, 25> = ArrayVec::new();
    let bytes: &[u8] = bytemuck::bytes_of(&swap_params);
    data.try_extend_from_slice(bytes);
    data.push(HUMIDIFI_SWAP_SELECTOR);

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

    Ok(())
}

program_entrypoint!(process_instruction);

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

    let amount_in: u64 = u64::from_le_bytes(array);
    process_humidifi_swap(accounts, amount_in)
}
