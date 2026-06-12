use std::{cmp::Ordering, net::Ipv4Addr};

use cidr::Ipv4Cidr;

use crate::{combiner::Ipv4CidrCombiner, Ipv4CidrSize};

/// To divide an IPv4 CIDR into subnetworks.
#[derive(Debug)]
pub struct Ipv4CidrSeparator;

impl Ipv4CidrSeparator {
    /// Evenly divide an IPv4 CIDR into a specific number of subnetworks.
    pub fn divide_by(cidr: &Ipv4Cidr, n: usize) -> Option<Vec<Ipv4CidrCombiner>> {
        let size = cidr.size();

        let n_u64 = u64::try_from(n).ok()?;

        if n == 0 || n_u64 > size {
            return None;
        } else if n == 1 {
            let mut combiner = Ipv4CidrCombiner::with_capacity(1);

            combiner.push(*cidr);

            return Some(vec![combiner]);
        }

        let mut output = Vec::new();

        output.try_reserve_exact(n).ok()?;

        let d = size / n_u64;

        let mut next_address = cidr.first_address().to_bits() as u64;
        let mut remaining_size = size;

        for index in 0..n {
            let chunk_size = if index + 1 == n { remaining_size } else { d };

            output.push(ipv4_cidr_combiner_from_range(next_address, chunk_size));

            if index + 1 != n {
                next_address += chunk_size;
                remaining_size -= chunk_size;
            }
        }

        Some(output)
    }

    /// Divide an IPv4 CIDR into subnetworks with a specific bits.
    pub fn sub_networks(cidr: &Ipv4Cidr, bits: u8) -> Option<Vec<Ipv4Cidr>> {
        let cidr_bits = cidr.network_length();

        if bits > 32 {
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

        let step = 1u32.checked_shl((32 - bits) as u32)?;
        let mut next_address = cidr.first_address().to_bits();

        for index in 0..n {
            output.push(Ipv4Cidr::new(Ipv4Addr::from(next_address), bits).unwrap());

            if index + 1 != n {
                next_address = next_address.checked_add(step)?;
            }
        }

        Some(output)
    }
}

fn ipv4_cidr_combiner_from_range(mut next_address: u64, mut size: u64) -> Ipv4CidrCombiner {
    let mut combiner = Ipv4CidrCombiner::new();

    while size > 0 {
        let alignment_bits =
            if next_address == 0 { 32 } else { (next_address as u32).trailing_zeros() };
        let size_bits = 63 - size.leading_zeros();
        let block_bits = alignment_bits.min(size_bits);
        let block_size = 1u64 << block_bits;
        let bits = 32 - block_bits as u8;

        combiner.push(Ipv4Cidr::new(Ipv4Addr::from(next_address as u32), bits).unwrap());

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
