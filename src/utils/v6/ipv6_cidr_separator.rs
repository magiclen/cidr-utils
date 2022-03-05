use std::cmp::Ordering;

use crate::cidr::Ipv6Cidr;
use crate::num_bigint::BigUint;
use crate::utils::Ipv6CidrCombiner;

use num_traits::{One, ToPrimitive};

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

        let log2_n = (n as f64).log2();

        let mut output = Vec::with_capacity(n);

        if (log2_n - log2_n.floor()).abs() < 2.0 * std::f64::EPSILON {
            let mut iter = cidr.iter();

            let bits = cidr.get_bits() + log2_n as u8;

            let usize_max_big_int = BigUint::from(usize::max_value());

            let d = size / n_big_int;

            if d <= usize_max_big_int {
                for ip in iter.step_by(d.to_usize().unwrap()) {
                    let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                    combiner.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());

                    output.push(combiner);
                }
            } else {
                let nth = d - BigUint::one();

                if let Some(ip) = iter.next() {
                    let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                    combiner.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());

                    output.push(combiner);

                    while let Some(ip) = iter.nth_big_uint(nth.clone()) {
                        let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                        combiner.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());

                        output.push(combiner);
                    }
                }
            }
        } else {
            let d = size / n_big_int;

            let iter = cidr.iter();

            let mut current_combiner = Ipv6CidrCombiner::new();

            let mut i = BigUint::one();

            for ip in iter {
                current_combiner.push(Ipv6Cidr::from_prefix_and_bits(ip, 128).unwrap());

                if i == d {
                    output.push(current_combiner);

                    current_combiner = Ipv6CidrCombiner::new();

                    i = BigUint::one();
                } else {
                    i += BigUint::one();
                }
            }

            let last_combiner = output.last_mut().unwrap();

            for cidr in current_combiner.into_ipv6_cidr_vec().into_iter() {
                last_combiner.push(cidr);
            }
        }

        Some(output)
    }

    /// Divide an IPv6 CIDR into subnetworks with a specific bits.
    pub fn sub_networks(cidr: &Ipv6Cidr, bits: u8) -> Option<Vec<Ipv6Cidr>> {
        let cidr_bits = cidr.get_bits();

        match cidr_bits.cmp(&bits) {
            Ordering::Greater => return None,
            Ordering::Equal => return Some(vec![*cidr]),
            Ordering::Less => (),
        }

        let n = 2usize.pow(u32::from(bits - cidr_bits));

        let n_big_int = BigUint::from(n);

        let mut output = Vec::with_capacity(n);

        let size = cidr.size();

        let d = size / n_big_int;

        let mut iter = cidr.iter();

        let usize_max_big_int = BigUint::from(usize::max_value());

        if d <= usize_max_big_int {
            for ip in iter.step_by(d.to_usize().unwrap()) {
                output.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());
            }
        } else {
            let nth = d - BigUint::one();

            if let Some(ip) = iter.next() {
                output.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());

                while let Some(ip) = iter.nth_big_uint(nth.clone()) {
                    output.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());
                }
            }
        }

        Some(output)
    }
}
