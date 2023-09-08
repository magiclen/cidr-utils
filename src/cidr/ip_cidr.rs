use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Debug, Display, Formatter},
    net::IpAddr,
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::de::{Deserialize, Deserializer, Error as DeError, Visitor};
#[cfg(feature = "serde")]
use serde::ser::{Serialize, Serializer};

use super::{IpCidrError, Ipv4Cidr, Ipv4CidrError, Ipv6Cidr, Ipv6CidrError};
use crate::num_bigint::BigUint;

// The type which can be taken as an IP address.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IpCidr {
    V4(Ipv4Cidr),
    V6(Ipv6Cidr),
}

impl IpCidr {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<IpCidr, IpCidrError> {
        let s = s.as_ref();

        match Ipv4Cidr::from_str(s) {
            Ok(cidr) => Ok(IpCidr::V4(cidr)),
            Err(err) => match err {
                Ipv4CidrError::IncorrectBitsRange => Err(IpCidrError::IncorrectBitsRange),
                Ipv4CidrError::IncorrectMask => Err(IpCidrError::IncorrectMask),
                Ipv4CidrError::IncorrectIpv4CIDRString => match Ipv6Cidr::from_str(s) {
                    Ok(cidr) => Ok(IpCidr::V6(cidr)),
                    Err(err) => match err {
                        Ipv6CidrError::IncorrectBitsRange => Err(IpCidrError::IncorrectBitsRange),
                        Ipv6CidrError::IncorrectMask => Err(IpCidrError::IncorrectMask),
                        Ipv6CidrError::IncorrectIpv6CIDRString => {
                            Err(IpCidrError::IncorrectIpCIDRString)
                        },
                    },
                },
            },
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
        Ipv6Cidr::from_str(s).is_ok()
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
    pub fn size(&self) -> BigUint {
        match self {
            IpCidr::V4(cidr) => BigUint::from(cidr.size()),
            IpCidr::V6(cidr) => cidr.size(),
        }
    }
}

impl IpCidr {
    #[inline]
    pub fn contains(&self, ip: IpAddr) -> bool {
        match self {
            IpCidr::V4(cidr) => match ip {
                IpAddr::V4(ip) => cidr.contains(ip),
                IpAddr::V6(_) => false,
            },
            IpCidr::V6(cidr) => match ip {
                IpAddr::V4(_) => false,
                IpAddr::V6(ip) => cidr.contains(ip),
            },
        }
    }
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

impl PartialEq<Ipv4Cidr> for IpCidr {
    #[inline]
    fn eq(&self, other: &Ipv4Cidr) -> bool {
        match self {
            IpCidr::V4(cidr) => cidr.eq(other),
            IpCidr::V6(_) => false,
        }
    }
}

impl PartialEq<IpCidr> for Ipv4Cidr {
    #[inline]
    fn eq(&self, other: &IpCidr) -> bool {
        match other {
            IpCidr::V4(cidr) => self.eq(cidr),
            IpCidr::V6(_) => false,
        }
    }
}

impl PartialOrd<Ipv4Cidr> for IpCidr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Cidr) -> Option<Ordering> {
        match self {
            IpCidr::V4(cidr) => cidr.partial_cmp(other),
            IpCidr::V6(_) => Some(Ordering::Greater),
        }
    }
}

impl PartialOrd<IpCidr> for Ipv4Cidr {
    #[inline]
    fn partial_cmp(&self, other: &IpCidr) -> Option<Ordering> {
        match other {
            IpCidr::V4(cidr) => self.partial_cmp(cidr),
            IpCidr::V6(_) => Some(Ordering::Less),
        }
    }
}

impl PartialEq<Ipv6Cidr> for IpCidr {
    #[inline]
    fn eq(&self, other: &Ipv6Cidr) -> bool {
        match self {
            IpCidr::V4(_) => false,
            IpCidr::V6(cidr) => cidr.eq(other),
        }
    }
}

impl PartialEq<IpCidr> for Ipv6Cidr {
    #[inline]
    fn eq(&self, other: &IpCidr) -> bool {
        match other {
            IpCidr::V4(_) => false,
            IpCidr::V6(cidr) => self.eq(cidr),
        }
    }
}

impl PartialOrd<Ipv6Cidr> for IpCidr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Cidr) -> Option<Ordering> {
        match self {
            IpCidr::V4(_) => Some(Ordering::Less),
            IpCidr::V6(cidr) => cidr.partial_cmp(other),
        }
    }
}

impl PartialOrd<IpCidr> for Ipv6Cidr {
    #[inline]
    fn partial_cmp(&self, other: &IpCidr) -> Option<Ordering> {
        match other {
            IpCidr::V4(_) => Some(Ordering::Greater),
            IpCidr::V6(cidr) => self.partial_cmp(cidr),
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

impl TryFrom<&str> for IpCidr {
    type Error = IpCidrError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        IpCidr::from_str(s)
    }
}

#[cfg(feature = "serde")]
impl Serialize for IpCidr {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for IpCidr {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct IpVisitor;

        impl<'de> Visitor<'de> for IpVisitor {
            type Value = IpCidr;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                f.write_str("a CIDR string")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeError, {
                IpCidr::from_str(v).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_str(IpVisitor)
    }
}
