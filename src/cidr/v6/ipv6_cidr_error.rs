use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Possible errors of `Ipv6Cidr`.
pub enum Ipv6CidrError {
    IncorrectBitsRange,
    IncorrectMask,
    IncorrectIpv6CIDRString,
}

impl Display for Ipv6CidrError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Ipv6CidrError::IncorrectBitsRange => {
                f.write_str("The subnet size (bits) is out of range.")
            }
            Ipv6CidrError::IncorrectMask => f.write_str("The mask is incorrect."),
            Ipv6CidrError::IncorrectIpv6CIDRString => {
                f.write_str("The CIDR (IPv6) string is incorrect.")
            }
        }
    }
}

impl Error for Ipv6CidrError {}
