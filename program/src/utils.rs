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
// a fex examples of humididif param account ,they nexis in on chain
//4VxJpqY3KnLCVZkjJASshXkWWoZxXKUqEdqv9X1tTtEb
//EcqJszWXUhmUFKFyjJcaGkE2FuwhhEcCLr45kGWqrY87
//DA4LYToRwnniNaXtZajdotkginSS2t5VkzWpREURTu5R
//BnxGGWoX1ACF85uUY5NDB5hNRHp4i9BjZJud2B41Lzcw
//7NDEUM3qnVCbQihkpkFnRttzQRDq5KzoCdEodFCeitWj
//EVNy5boNfAcFZq1NU8sL6h8x57Ce5NxDmF3UA7R7926T

//3aff2fffe2baebc32e84af4de3baeac338ff2dffe0bae9c33d
//3aff2fffe2baebc33ba1fe4de3baeac338ff2dffe0bae9c33d
//3aff2fffe2baebc3daa95feee3baeac339ff2dffe0bae9c33d
//3aff2fffe2baebc3d841994de3baeac338ff2dffe0bae9c33d
//3aff2fffe2baebc32e84af4de3baeac338ff2dffe0bae9c33d
//3aff2fffe2baebc376ee55e3e3baeac339ff2dffe0bae9c33d
//3aff2fffe2baebc3ced967a5e3baeac339ff2dffe0bae9c33d
//3aff2fffe2baebc3c496f9c7e3baeac339ff2dffe0bae9c33d
//king
//df280b769cc7baada4a8f2bcf7baeac338ff2dffe0bae9c33d
//cb3eb334440af28af719088af8baeac338ff2dffe0bae9c33d

pub fn spin_instruction_data(data: &mut [u8]) {
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

//38 b3 67 75 d4 ba 51 f9
//60 8f 65 15 b8 73 1f 87
//58 62 17 3b 92 e2 da b8
//41 2c 40 c6 c0 21 5c 07
//15 9b af cf 46 c8 31 8f
