use std::cmp::Ordering;

use crate::{cidr::Ipv4Cidr, utils::Ipv4CidrCombiner};

/// To divide an IPv4 CIDR into subnetworks.
#[derive(Debug)]
pub struct Ipv4CidrSeparator;

impl Ipv4CidrSeparator {
    /// Evenly divide an IPv4 CIDR into a specific number of subnetworks.
    pub fn divide_by(cidr: &Ipv4Cidr, n: usize) -> Option<Vec<Ipv4CidrCombiner>> {
        let size = cidr.size();

        let n_u64 = n as u64;

        if n == 0 || n_u64 > size {
            return None;
        } else if n == 1 {
            let mut combiner = Ipv4CidrCombiner::with_capacity(1);

            combiner.push(*cidr);

            return Some(vec![combiner]);
        }

        let mut output = Vec::with_capacity(n);

        let d = size / n_u64;

        if d * n_u64 == size {
            let mut iter = cidr.iter();

            let bits = cidr.get_bits() + (n as f64).log2() as u8;

            let usize_max_u64 = usize::MAX as u64;

            if d <= usize_max_u64 {
                for ip in iter.step_by(d as usize) {
                    let mut combiner = Ipv4CidrCombiner::with_capacity(1);

                    combiner.push(Ipv4Cidr::from_prefix_and_bits(ip, bits).unwrap());

                    output.push(combiner);
                }
            } else {
                let nth = d - 1;

                if let Some(ip) = iter.next() {
                    let mut combiner = Ipv4CidrCombiner::with_capacity(1);

                    combiner.push(Ipv4Cidr::from_prefix_and_bits(ip, bits).unwrap());

                    output.push(combiner);

                    while let Some(ip) = iter.nth_u64(nth) {
                        let mut combiner = Ipv4CidrCombiner::with_capacity(1);

                        combiner.push(Ipv4Cidr::from_prefix_and_bits(ip, bits).unwrap());

                        output.push(combiner);
                    }
                }
            }
        } else {
            let iter = cidr.iter();

            let mut current_combiner = Ipv4CidrCombiner::new();

            let mut i = 1;

            for ip in iter {
                current_combiner.push(Ipv4Cidr::from_prefix_and_bits(ip, 32).unwrap());

                if i == d {
                    output.push(current_combiner);

                    current_combiner = Ipv4CidrCombiner::new();

                    i = 1;
                } else {
                    i += 1;
                }
            }

            let last_combiner = output.last_mut().unwrap();

            for cidr in current_combiner.into_ipv4_cidr_vec().into_iter() {
                last_combiner.push(cidr);
            }
        }

        Some(output)
    }

    /// Divide an IPv4 CIDR into subnetworks with a specific bits.
    pub fn sub_networks(cidr: &Ipv4Cidr, bits: u8) -> Option<Vec<Ipv4Cidr>> {
        let cidr_bits = cidr.get_bits();

        match cidr_bits.cmp(&bits) {
            Ordering::Greater => return None,
            Ordering::Equal => return Some(vec![*cidr]),
            Ordering::Less => (),
        }

        let n = 2usize.pow(u32::from(bits - cidr_bits));

        let n_u64 = n as u64;

        let mut output = Vec::with_capacity(n);

        let d = cidr.size() / n_u64;

        let mut iter = cidr.iter();

        let usize_max_u64 = usize::MAX as u64;

        if d <= usize_max_u64 {
            for ip in iter.step_by(d as usize) {
                output.push(Ipv4Cidr::from_prefix_and_bits(ip, bits).unwrap());
            }
        } else {
            let nth = d - 1;

            if let Some(ip) = iter.next() {
                output.push(Ipv4Cidr::from_prefix_and_bits(ip, bits).unwrap());

                while let Some(ip) = iter.nth_u64(nth) {
                    output.push(Ipv4Cidr::from_prefix_and_bits(ip, bits).unwrap());
                }
            }
        }

        Some(output)
    }
}
