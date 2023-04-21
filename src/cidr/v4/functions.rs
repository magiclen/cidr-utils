#[inline]
pub(in crate::cidr::v4) fn get_mask(bits: u8) -> u32 {
    let mut b = 0;

    for _ in 0..bits {
        b = (1 << 31) | (b >> 1);
    }

    b
}

pub(in crate::cidr::v4) fn mask_to_bits(mask: u32) -> Option<u8> {
    let mut digit = 31;

    loop {
        if (mask >> digit) & 1 == 0 {
            let b = 31 - digit as u8;

            for digit in (0..digit).rev() {
                if (mask >> digit) & 1 == 1 {
                    return None;
                }
            }

            return Some(b);
        }

        if digit == 1 {
            // check digit = 0
            return if mask & 1 == 1 { Some(32) } else { Some(31) };
        }

        digit -= 1;
    }
}
