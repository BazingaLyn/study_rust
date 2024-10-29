use std::intrinsics::copy_nonoverlapping;

pub(crate) fn decode_fixed_64(src: &[u8]) -> u64 {
    let mut data: u64 = 0;
    if src.len() >= 8 {
        unsafe {
            copy_nonoverlapping(src.as_ptr(), &mut data as *mut u64 as *mut u8, 8);
        }
    } else {
        for (i, b) in src.iter().enumerate() {
            data += (u64::from(*b)) << (i * 8);
        }
    }
    data.to_le()
}