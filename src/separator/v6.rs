use std::cmp::Ordering;

use cidr::Ipv6Cidr;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};

use crate::{combiner::Ipv6CidrCombiner, iterator::Ipv6CidrIpv6AddrIterator, Ipv6CidrSize};

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

        let d = size.clone() / n_big_int.clone();

        let mut output = Vec::with_capacity(n);

        if d.clone() * n_big_int == size {
            let mut iter = Ipv6CidrIpv6AddrIterator::new(cidr);

            let bits = cidr.network_length() + n.ilog2() as u8;

            let usize_max_big_int = BigUint::from(usize::MAX);

            if d <= usize_max_big_int {
                for ip in iter.step_by(d.to_usize().unwrap()) {
                    let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                    combiner.push(Ipv6Cidr::new(ip, bits).unwrap());

                    output.push(combiner);
                }
            } else {
                let nth = d - BigUint::one();

                if let Some(ip) = iter.next() {
                    let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                    combiner.push(Ipv6Cidr::new(ip, bits).unwrap());

                    output.push(combiner);

                    while let Some(ip) = iter.nth_big_uint(nth.clone()) {
                        let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                        combiner.push(Ipv6Cidr::new(ip, bits).unwrap());

                        output.push(combiner);
                    }
                }
            }
        } else {
            let iter = Ipv6CidrIpv6AddrIterator::new(cidr);

            let mut current_combiner = Ipv6CidrCombiner::new();

            let mut i = BigUint::one();

            for ip in iter {
                current_combiner.push(Ipv6Cidr::new(ip, 128).unwrap());

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
        let cidr_bits = cidr.network_length();

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

        let mut iter = Ipv6CidrIpv6AddrIterator::new(cidr);

        let usize_max_big_int = BigUint::from(usize::MAX);

        if d <= usize_max_big_int {
            for ip in iter.step_by(d.to_usize().unwrap()) {
                output.push(Ipv6Cidr::new(ip, bits).unwrap());
            }
        } else {
            let nth = d - BigUint::one();

            if let Some(ip) = iter.next() {
                output.push(Ipv6Cidr::new(ip, bits).unwrap());

                while let Some(ip) = iter.nth_big_uint(nth.clone()) {
                    output.push(Ipv6Cidr::new(ip, bits).unwrap());
                }
            }
        }

        Some(output)
    }
}
