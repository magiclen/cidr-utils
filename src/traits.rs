use cidr::{Ipv4Cidr, Ipv6Cidr};
use num_bigint::BigUint;

/// Get the size of an Ipv4Cidr.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use cidr::Ipv4Cidr;
/// use cidr_utils::Ipv4CidrSize;
///
/// let cidr = Ipv4Cidr::from_str("192.168.1.0/24").unwrap();
///
/// assert_eq!(256, cidr.size());
/// ```
pub trait Ipv4CidrSize {
    fn size(&self) -> u64;
}

impl Ipv4CidrSize for Ipv4Cidr {
    #[inline]
    fn size(&self) -> u64 {
        2u64.pow((32 - self.network_length()) as u32)
    }
}

/// Get the size of an Ipv6Cidr.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use cidr::Ipv6Cidr;
/// use cidr_utils::Ipv6CidrSize;
/// use num_bigint::BigUint;
///
/// let cidr = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();
///
/// assert_eq!(BigUint::from(65536usize), cidr.size());
/// ```
pub trait Ipv6CidrSize {
    fn size(&self) -> BigUint;
}

impl Ipv6CidrSize for Ipv6Cidr {
    #[inline]
    fn size(&self) -> BigUint {
        BigUint::from(2u8).pow((128 - self.network_length()) as u32)
    }
}
