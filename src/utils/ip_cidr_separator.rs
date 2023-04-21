use crate::{
    cidr::IpCidr,
    utils::{IpCidrCombiner, Ipv4CidrSeparator, Ipv6CidrSeparator},
};

/// To divide an IP CIDR into subnetworks.
#[derive(Debug)]
pub struct IpCidrSeparator;

impl IpCidrSeparator {
    /// Evenly divide an IP CIDR into a specific number of subnetworks.
    pub fn divide_by(cidr: &IpCidr, n: usize) -> Option<Vec<IpCidrCombiner>> {
        match cidr {
            IpCidr::V4(cidr) => Ipv4CidrSeparator::divide_by(cidr, n).map(|v| {
                v.into_iter()
                    .map(|combiner| unsafe {
                        IpCidrCombiner::from_cidr_vec_unchecked(
                            combiner.into_ipv4_cidr_vec(),
                            vec![],
                        )
                    })
                    .collect()
            }),
            IpCidr::V6(cidr) => Ipv6CidrSeparator::divide_by(cidr, n).map(|v| {
                v.into_iter()
                    .map(|combiner| unsafe {
                        IpCidrCombiner::from_cidr_vec_unchecked(
                            vec![],
                            combiner.into_ipv6_cidr_vec(),
                        )
                    })
                    .collect()
            }),
        }
    }

    /// Divide an IP CIDR into subnetworks with a specific bits.
    pub fn sub_networks(cidr: &IpCidr, bits: u8) -> Option<Vec<IpCidr>> {
        match cidr {
            IpCidr::V4(cidr) => Ipv4CidrSeparator::sub_networks(cidr, bits)
                .map(|v| v.into_iter().map(IpCidr::V4).collect()),
            IpCidr::V6(cidr) => Ipv6CidrSeparator::sub_networks(cidr, bits)
                .map(|v| v.into_iter().map(IpCidr::V6).collect()),
        }
    }
}
