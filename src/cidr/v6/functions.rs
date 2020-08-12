#[inline]
pub(in crate::cidr::v6) fn get_mask(bits: u8) -> u128 {
    let mut b = 0;

    for _ in 0..bits {
        b = (1 << 127) | (b >> 1);
    }

    b
}

pub(in crate::cidr::v6) fn mask_to_bits(mask: u128) -> Option<u8> {
    let mut digit = 127;

    loop {
        if (mask >> digit) & 1 == 0 {
            let b = 127 - digit as u8;

            for digit in (0..digit).rev() {
                if (mask >> digit) & 1 == 1 {
                    return None;
                }
            }

            return Some(b);
        }

        if digit == 1 {
            // check digit = 0
            return if mask & 1 == 1 {
                Some(128)
            } else {
                Some(127)
            };
        }

        digit -= 1;
    }
}

#[inline]
pub(in crate::cidr::v6) fn u128_to_u16_array(uint128: u128) -> [u16; 8] {
    let a = uint128.to_be_bytes();

    let mut o = [0; 8];

    for (i, e) in o.iter_mut().enumerate() {
        let ii = i * 2;

        *e = a[ii] as u16 * 256 + a[ii + 1] as u16;
    }

    o
}

pub(in crate::cidr::v6) fn u16_array_to_u128(uint16_array: [u16; 8]) -> u128 {
    let mut a = [0; 16];

    for (i, &e) in uint16_array.iter().enumerate() {
        let ii = i * 2;

        a[ii] = (e / 256) as u8;
        a[ii + 1] = (e % 256) as u8;
    }

    u128::from_be_bytes(a)
}
