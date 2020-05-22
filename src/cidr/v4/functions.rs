#[inline]
pub(in crate::cidr::v4) fn get_mask(bits: u8) -> u32 {
    let mut b = 0;

    for _ in 0..bits {
        b = 0x8000_0000 | (b >> 1);
    }

    b
}

#[inline]
pub(in crate::cidr::v4) fn u32_to_u8_array(uint32: u32) -> [u8; 4] {
    uint32.to_be_bytes()
}

#[inline]
pub(in crate::cidr::v4) fn u8_array_to_u32(uint8_array: [u8; 4]) -> u32 {
    u32::from_be_bytes(uint8_array)
}

pub(in crate::cidr::v4) fn mask_to_bits(mask: u32) -> Option<u8> {
    let mut digit = 0;
    let mut b = 32;

    for _ in 0..32 {
        let n = (mask << digit) >> 31;

        if n == 0 {
            b = digit as u8;
            break;
        }

        digit += 1;
    }

    for digit in digit..32 {
        let n = (mask << digit) >> 31;

        if n == 1 {
            return None;
        }
    }

    Some(b)
}
