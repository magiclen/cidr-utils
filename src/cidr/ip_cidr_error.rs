use std::error::Error;
use std::fmt::{self, Display, Formatter};

use super::{Ipv4CidrError, Ipv6CidrError};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
/// Possible errors of `IpCidr`.
pub enum IpCidrError {
    IncorrectBitsRange,
    IncorrectMask,
    IncorrectIpCIDRString,
}

impl From<Ipv4CidrError> for IpCidrError {
    #[inline]
    fn from(error: Ipv4CidrError) -> IpCidrError {
        unsafe { IpCidrError::from_ordinal_unsafe(error.ordinal()) }
    }
}

impl From<Ipv6CidrError> for IpCidrError {
    #[inline]
    fn from(error: Ipv6CidrError) -> IpCidrError {
        unsafe { IpCidrError::from_ordinal_unsafe(error.ordinal()) }
    }
}

impl Display for IpCidrError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IpCidrError::IncorrectBitsRange => {
                f.write_str("The subnet size (bits) is out of range.")
            }
            IpCidrError::IncorrectMask => f.write_str("The mask is incorrect."),
            IpCidrError::IncorrectIpCIDRString => f.write_str("The CIDR string is incorrect."),
        }
    }
}

impl Error for IpCidrError {}
