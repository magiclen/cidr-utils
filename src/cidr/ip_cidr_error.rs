use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use super::{Ipv4CidrError, Ipv6CidrError};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Possible errors of `IpCidr`.
pub enum IpCidrError {
    IncorrectBitsRange,
    IncorrectMask,
    IncorrectIpCIDRString,
}

impl From<Ipv4CidrError> for IpCidrError {
    #[inline]
    fn from(error: Ipv4CidrError) -> IpCidrError {
        match error {
            Ipv4CidrError::IncorrectBitsRange => IpCidrError::IncorrectBitsRange,
            Ipv4CidrError::IncorrectMask => IpCidrError::IncorrectMask,
            Ipv4CidrError::IncorrectIpv4CIDRString => IpCidrError::IncorrectIpCIDRString,
        }
    }
}

impl From<Ipv6CidrError> for IpCidrError {
    #[inline]
    fn from(error: Ipv6CidrError) -> IpCidrError {
        match error {
            Ipv6CidrError::IncorrectBitsRange => IpCidrError::IncorrectBitsRange,
            Ipv6CidrError::IncorrectMask => IpCidrError::IncorrectMask,
            Ipv6CidrError::IncorrectIpv6CIDRString => IpCidrError::IncorrectIpCIDRString,
        }
    }
}

impl Display for IpCidrError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IpCidrError::IncorrectBitsRange => {
                f.write_str("The subnet size (bits) is out of range.")
            },
            IpCidrError::IncorrectMask => f.write_str("The mask is incorrect."),
            IpCidrError::IncorrectIpCIDRString => f.write_str("The CIDR string is incorrect."),
        }
    }
}

impl Error for IpCidrError {}
