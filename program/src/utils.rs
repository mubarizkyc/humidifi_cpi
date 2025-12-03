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

pub fn obfuscate_instruction_data(data: &mut [u8]) {
    let mut qwords = data.chunks_exact_mut(8);
    let mut pos_mask = 0_u64;
    while let Some(qword) = qwords
        .next()
        .map(|q| unsafe { &mut *q.as_mut_ptr().cast::<u64>() })
    {
        *qword ^= HUMIDIFI_IX_DATA_KEY;
        *qword ^= pos_mask;
        pos_mask = pos_mask.wrapping_add(0x0001_0001_0001_0001);
    }
    let remainder = qwords.into_remainder();
    let mut rem = 0_u64;
    unsafe {
        core::ptr::copy_nonoverlapping(
            remainder.as_ptr(),
            &mut rem as *mut u64 as *mut u8,
            remainder.len(),
        );
    }
    rem ^= HUMIDIFI_IX_DATA_KEY;
    rem ^= pos_mask;
    unsafe {
        core::ptr::copy_nonoverlapping(
            &rem as *const u64 as *const u8,
            remainder.as_mut_ptr(),
            remainder.len(),
        )
    }
}
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SwapParams {
    pub swap_id: u64,
    pub amount_in: u64,
    pub is_base_to_quote: u8,
    pub padding: [u8; 7],
}
