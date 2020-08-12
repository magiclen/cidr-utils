use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Possible errors of `Ipv4Cidr`.
pub enum Ipv4CidrError {
    IncorrectBitsRange,
    IncorrectMask,
    IncorrectIpv4CIDRString,
}

impl Display for Ipv4CidrError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Ipv4CidrError::IncorrectBitsRange => {
                f.write_str("The subnet size (bits) is out of range.")
            }
            Ipv4CidrError::IncorrectMask => f.write_str("The mask is incorrect."),
            Ipv4CidrError::IncorrectIpv4CIDRString => {
                f.write_str("The CIDR (IPv4) string is incorrect.")
            }
        }
    }
}

impl Error for Ipv4CidrError {}
