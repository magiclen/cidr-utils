use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::net::IpAddr;
use std::str::FromStr;

use crate::cidr::ipv4_cidr::{Ipv4Cidr, Ipv4CidrIpv4AddrIterator};
use crate::cidr::ipv6_cidr::{Ipv6Cidr, Ipv6CidrIpv6AddrIterator};

// TODO: IpCidr

#[derive(Debug, PartialEq, Eq, Clone)]
// The type which can be taken as an IP address.
pub enum IpCidr {
    V4(Ipv4Cidr),
    V6(Ipv6Cidr),
}

impl Display for IpCidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            IpCidr::V4(cidr) => Display::fmt(&cidr, f),
            IpCidr::V6(cidr) => Display::fmt(&cidr, f),
        }
    }
}

impl PartialEq<Ipv6Cidr> for IpCidr {
    #[inline]
    fn eq(&self, other: &Ipv6Cidr) -> bool {
        match self {
            IpCidr::V4(_) => false,
            IpCidr::V6(cidr) => cidr.eq(&other),
        }
    }
}

impl PartialOrd<Ipv6Cidr> for IpCidr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Cidr) -> Option<Ordering> {
        match self {
            IpCidr::V4(_) => Some(Ordering::Less),
            IpCidr::V6(cidr) => cidr.partial_cmp(&other),
        }
    }
}

impl PartialEq<Ipv4Cidr> for IpCidr {
    #[inline]
    fn eq(&self, other: &Ipv4Cidr) -> bool {
        match self {
            IpCidr::V4(cidr) => cidr.eq(&other),
            IpCidr::V6(_) => false,
        }
    }
}

impl PartialOrd<Ipv4Cidr> for IpCidr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Cidr) -> Option<Ordering> {
        match self {
            IpCidr::V4(cidr) => cidr.partial_cmp(&other),
            IpCidr::V6(_) => Some(Ordering::Greater),
        }
    }
}

impl PartialOrd for IpCidr {
    #[inline]
    fn partial_cmp(&self, other: &IpCidr) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for IpCidr {
    #[inline]
    fn cmp(&self, other: &IpCidr) -> Ordering {
        match other {
            IpCidr::V4(cidr) => self.partial_cmp(cidr).unwrap(),
            IpCidr::V6(cidr) => self.partial_cmp(cidr).unwrap(),
        }
    }
}

impl FromStr for IpCidr {
    type Err = IpCidrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        IpCidr::from_str(s)
    }
}

#[derive(Debug, PartialEq)]
/// Possible errors of `IpCidr`.
pub enum IpCidrError {
    IncorrectBitsRange,
    IncorrectMask,
    IncorrectIpCIDRString,
}

impl fmt::Display for IpCidrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IpCidrError::IncorrectBitsRange => write!(f, "Incorrect bits range"),
            IpCidrError::IncorrectMask => write!(f, "Incorrect mask"),
            IpCidrError::IncorrectIpCIDRString => write!(f, "Incorrect IpCIDR string"),
        }
    }
}

impl IpCidr {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<IpCidr, IpCidrError> {
        let s = s.as_ref();

        match Ipv4Cidr::from_str(s) {
            Ok(cidr) => Ok(IpCidr::V4(cidr)),
            Err(_) => {
                match Ipv6Cidr::from_str(s) {
                    Ok(cidr) => Ok(IpCidr::V6(cidr)),
                    Err(_) => Err(IpCidrError::IncorrectIpCIDRString),
                }
            }
        }
    }

    #[inline]
    pub fn is_ip_cidr<S: AsRef<str>>(s: S) -> bool {
        Self::from_str(s).is_ok()
    }

    #[inline]
    pub fn is_ipv4_cidr<S: AsRef<str>>(s: S) -> bool {
        Ipv4Cidr::from_str(s).is_ok()
    }

    #[inline]
    pub fn is_ipv6_cidr<S: AsRef<str>>(s: S) -> bool {
        Ipv4Cidr::from_str(s).is_ok()
    }
}

impl IpCidr {
    #[inline]
    pub fn first_as_ip_addr(&self) -> IpAddr {
        match self {
            IpCidr::V4(cidr) => IpAddr::V4(cidr.first_as_ipv4_addr()),
            IpCidr::V6(cidr) => IpAddr::V6(cidr.first_as_ipv6_addr()),
        }
    }

    #[inline]
    pub fn last_as_ip_addr(&self) -> IpAddr {
        match self {
            IpCidr::V4(cidr) => IpAddr::V4(cidr.last_as_ipv4_addr()),
            IpCidr::V6(cidr) => IpAddr::V6(cidr.last_as_ipv6_addr()),
        }
    }

    #[inline]
    pub fn size(&self) -> (u128, bool) {
        match self {
            IpCidr::V4(cidr) => (u128::from(cidr.size()), false),
            IpCidr::V6(cidr) => cidr.size(),
        }
    }
}

impl IpCidr {
    #[inline]
    pub fn contains(&self, ip: IpAddr) -> bool {
        match self {
            IpCidr::V4(cidr) => {
                match ip {
                    IpAddr::V4(ip) => cidr.contains(ip),
                    IpAddr::V6(_) => false,
                }
            }
            IpCidr::V6(cidr) => {
                match ip {
                    IpAddr::V4(_) => false,
                    IpAddr::V6(ip) => cidr.contains(ip),
                }
            }
        }
    }
}

// TODO: IpCidrIpAddrIterator
#[derive(Debug)]
enum IpCidrIpsAddrIterator {
    V4(Ipv4CidrIpv4AddrIterator),
    V6(Ipv6CidrIpv6AddrIterator),
}

/// To iterate IP CIDRs.
#[derive(Debug)]
pub struct IpCidrIpAddrIterator {
    iter: IpCidrIpsAddrIterator,
}

impl Iterator for IpCidrIpAddrIterator {
    type Item = IpAddr;

    #[inline]
    fn next(&mut self) -> Option<IpAddr> {
        match &mut self.iter {
            IpCidrIpsAddrIterator::V4(iter) => iter.next().map(IpAddr::V4),
            IpCidrIpsAddrIterator::V6(iter) => iter.next().map(IpAddr::V6),
        }
    }

    #[inline]
    fn last(self) -> Option<IpAddr> {
        match self.iter {
            IpCidrIpsAddrIterator::V4(iter) => iter.last().map(IpAddr::V4),
            IpCidrIpsAddrIterator::V6(iter) => iter.last().map(IpAddr::V6),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<IpAddr> {
        match &mut self.iter {
            IpCidrIpsAddrIterator::V4(iter) => iter.nth(n).map(IpAddr::V4),
            IpCidrIpsAddrIterator::V6(iter) => iter.nth(n).map(IpAddr::V6),
        }
    }
}

impl IpCidr {
    #[inline]
    pub fn iter_as_ip_addr(&self) -> IpCidrIpAddrIterator {
        match self {
            IpCidr::V4(cidr) => {
                IpCidrIpAddrIterator {
                    iter: IpCidrIpsAddrIterator::V4(cidr.iter_as_ipv4_addr()),
                }
            }
            IpCidr::V6(cidr) => {
                IpCidrIpAddrIterator {
                    iter: IpCidrIpsAddrIterator::V6(cidr.iter_as_ipv6_addr()),
                }
            }
        }
    }

    #[inline]
    pub fn iter(&self) -> IpCidrIpAddrIterator {
        self.iter_as_ip_addr()
    }
}
