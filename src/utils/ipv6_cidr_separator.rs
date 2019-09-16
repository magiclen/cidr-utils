use crate::cidr::Ipv6Cidr;
use crate::utils::Ipv6CidrCombiner;

/// To divide an IPv6 CIDR into subnetworks.
pub struct Ipv6CidrSeparator {}

impl Ipv6CidrSeparator {
    /// Evenly divide an IPv6 CIDR into a specific number of subnetworks.
    pub fn divide_by(cidr: &Ipv6Cidr, n: usize) -> Option<Vec<Ipv6CidrCombiner>> {
        let size = cidr.size();

        let n_u128 = n as u128;

        if n == 0 || (!size.1 && n_u128 > size.0) {
            return None;
        } else if n == 1 {
            let mut combiner = Ipv6CidrCombiner::with_capacity(1);

            combiner.push(cidr.clone());

            return Some(vec![combiner]);
        }

        let log2_n = (n as f64).log2();

        let mut output = Vec::with_capacity(n);

        if (log2_n - log2_n.floor()).abs() < 2.0 * std::f64::EPSILON {
            let mut iter = cidr.iter();

            let bits = cidr.get_bits() + log2_n as u8;

            let usize_max_u128 = usize::max_value() as u128;

            let d = if size.1 {
                u128::max_value() / n_u128 + 1
            } else {
                size.0 / n_u128
            };

            if d <= usize_max_u128 {
                for ip in iter.step_by(d as usize) {
                    let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                    combiner.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());

                    output.push(combiner);
                }
            } else {
                let nth = d - 1;

                if let Some(ip) = iter.next() {
                    let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                    combiner.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());

                    output.push(combiner);

                    while let Some(ip) = iter.nth_u128((nth, false)) {
                        let mut combiner = Ipv6CidrCombiner::with_capacity(1);

                        combiner.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());

                        output.push(combiner);
                    }
                }
            }
        } else {
            let d = if size.1 {
                u128::max_value() / n_u128
            } else {
                size.0 / n_u128
            };

            let iter = cidr.iter();

            let mut current_combiner = Ipv6CidrCombiner::new();

            let mut i = 1;

            for ip in iter {
                current_combiner.push(Ipv6Cidr::from_prefix_and_bits(ip, 128).unwrap());

                if i == d {
                    output.push(current_combiner);

                    current_combiner = Ipv6CidrCombiner::new();

                    i = 1;
                } else {
                    i += 1;
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

        if cidr_bits > bits {
            return None;
        } else if cidr_bits == bits {
            return Some(vec![cidr.clone()]);
        }

        let n = 2usize.pow(u32::from(bits - cidr_bits));

        let n_u128 = n as u128;

        let mut output = Vec::with_capacity(n);

        let size = cidr.size();

        let d = if size.1 {
            u128::max_value() / n_u128 + 1
        } else {
            size.0 / n_u128
        };

        let mut iter = cidr.iter();

        let usize_max_u128 = usize::max_value() as u128;

        if d <= usize_max_u128 {
            for ip in iter.step_by(d as usize) {
                output.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());
            }
        } else {
            let nth = d - 1;

            if let Some(ip) = iter.next() {
                output.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());

                while let Some(ip) = iter.nth_u128((nth, false)) {
                    output.push(Ipv6Cidr::from_prefix_and_bits(ip, bits).unwrap());
                }
            }
        }

        Some(output)
    }
}
