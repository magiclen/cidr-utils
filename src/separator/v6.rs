use std::{cmp::Ordering, net::Ipv6Addr};

use cidr::Ipv6Cidr;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

use crate::{combiner::Ipv6CidrCombiner, Ipv6CidrSize};

/// To divide an IPv6 CIDR into subnetworks.
#[derive(Debug)]
pub struct Ipv6CidrSeparator;

impl Ipv6CidrSeparator {
    /// Evenly divide an IPv6 CIDR into a specific number of subnetworks.
    pub fn divide_by(cidr: &Ipv6Cidr, n: usize) -> Option<Vec<Ipv6CidrCombiner>> {
        let size = cidr.size();

        let n_big_int = BigUint::from(n);

        if n == 0 || n_big_int > size {
            return None;
        } else if n == 1 {
            let mut combiner = Ipv6CidrCombiner::with_capacity(1);

            combiner.push(*cidr);

            return Some(vec![combiner]);
        }

        let d = &size / &n_big_int;

        let mut output = Vec::new();

        output.try_reserve_exact(n).ok()?;

        let mut next_address = cidr.first_address().to_bits();
        let mut remaining_size = size;

        for index in 0..n {
            let chunk_size = if index + 1 == n { remaining_size.to_u128()? } else { d.to_u128()? };

            output.push(ipv6_cidr_combiner_from_range(next_address, chunk_size));

            if index + 1 != n {
                next_address = next_address.checked_add(chunk_size)?;
                remaining_size -= &d;
            }
        }

        Some(output)
    }

    /// Divide an IPv6 CIDR into subnetworks with a specific bits.
    pub fn sub_networks(cidr: &Ipv6Cidr, bits: u8) -> Option<Vec<Ipv6Cidr>> {
        let cidr_bits = cidr.network_length();

        if bits > 128 {
            return None;
        }

        match cidr_bits.cmp(&bits) {
            Ordering::Greater => return None,
            Ordering::Equal => return Some(vec![*cidr]),
            Ordering::Less => (),
        }

        let n = sub_network_count(bits - cidr_bits)?;

        let mut output = Vec::new();

        output.try_reserve_exact(n).ok()?;

        let step = 1u128.checked_shl((128 - bits) as u32)?;
        let mut next_address = cidr.first_address().to_bits();

        for index in 0..n {
            output.push(Ipv6Cidr::new(Ipv6Addr::from(next_address), bits).unwrap());

            if index + 1 != n {
                next_address = next_address.checked_add(step)?;
            }
        }

        Some(output)
    }
}

fn ipv6_cidr_combiner_from_range(mut next_address: u128, mut size: u128) -> Ipv6CidrCombiner {
    let mut combiner = Ipv6CidrCombiner::new();

    while size > 0 {
        let alignment_bits = if next_address == 0 { 128 } else { next_address.trailing_zeros() };
        let size_bits = 127 - size.leading_zeros();
        let block_bits = alignment_bits.min(size_bits);
        let block_size = 1u128 << block_bits;
        let bits = 128 - block_bits as u8;

        combiner.push(Ipv6Cidr::new(Ipv6Addr::from(next_address), bits).unwrap());

        size -= block_size;

        if size > 0 {
            next_address += block_size;
        }
    }

    combiner
}

#[inline]
const fn sub_network_count(bits: u8) -> Option<usize> {
    1usize.checked_shl(bits as u32)
}
