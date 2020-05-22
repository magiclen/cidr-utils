#[inline]
pub(in crate::cidr::v6) fn subtract(a: (u128, bool), b: (u128, bool)) -> (u128, bool) {
    if a.1 {
        if b.1 {
            (0, false)
        } else if b.0 == 0 {
            (0, true)
        } else {
            (u128::max_value() - b.0 + 1, false)
        }
    } else if b.1 {
        unreachable!()
    } else {
        (a.0 - b.0, false)
    }
}

#[inline]
pub(in crate::cidr::v6) fn get_mask(bits: u8) -> u128 {
    let mut b = 0;

    for _ in 0..bits {
        b = 0x8000_0000_0000_0000_0000_0000_0000_0000 | (b >> 1);
    }

    b
}

#[inline]
pub(in crate::cidr::v6) fn u128_to_u8_array(uint128: u128) -> [u8; 16] {
    uint128.to_be_bytes()
}

#[inline]
pub(in crate::cidr::v6) fn u128_to_u16_array(uint128: u128) -> [u16; 8] {
    let a = u128_to_u8_array(uint128);

    let mut o = [0; 8];

    for (i, e) in o.iter_mut().enumerate() {
        let ii = i * 2;

        *e = a[ii] as u16 * 256 + a[ii + 1] as u16;
    }

    o
}

#[inline]
pub(in crate::cidr::v6) fn u8_array_to_u128(uint8_array: [u8; 16]) -> u128 {
    u128::from_be_bytes(uint8_array)
}

pub(in crate::cidr::v6) fn u16_array_to_u128(uint16_array: [u16; 8]) -> u128 {
    let mut a = [0; 16];

    for (i, &e) in uint16_array.iter().enumerate() {
        let ii = i * 2;

        a[ii] = (e / 256) as u8;
        a[ii + 1] = (e % 256) as u8;
    }

    u8_array_to_u128(a)
}

pub(in crate::cidr::v6) fn mask_to_bits(mask: u128) -> Option<u8> {
    let mut digit = 0;
    let mut b = 128;

    for _ in 0..128 {
        let n = (mask << digit) >> 127;

        if n == 0 {
            b = digit as u8;
            break;
        }

        digit += 1;
    }

    for digit in digit..128 {
        let n = (mask << digit) >> 127;

        if n == 1 {
            return None;
        }
    }

    Some(b)
}
