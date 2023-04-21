use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Debug, Display, Formatter},
    net::Ipv6Addr,
    str::FromStr,
};

use once_cell::sync::Lazy;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::de::{Deserialize, Deserializer, Error as DeError, Visitor};
#[cfg(feature = "serde")]
use serde::ser::{Serialize, Serializer};

use super::{functions::*, Ipv6Able, Ipv6CidrError};
use crate::num_bigint::BigUint;

static RE_IPV6_CIDR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([^/]+)(?:/((?:12[0-8])|(?:1[0-1][0-9])|(?:[1-9][0-9])|[0-9]))?$").unwrap()
});

/// To represent IPv6 CIDR.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ipv6Cidr {
    prefix: u128,
    mask:   u128,
}

impl Ipv6Cidr {
    #[inline]
    /// Get an integer which represents the prefix an IPv6 byte array of this CIDR in big-endian (BE) order.
    pub fn get_prefix(&self) -> u128 {
        self.prefix
    }

    #[inline]
    pub fn get_prefix_as_u8_array(&self) -> [u8; 16] {
        self.get_prefix().to_be_bytes()
    }

    #[inline]
    pub fn get_prefix_as_u16_array(&self) -> [u16; 8] {
        u128_to_u16_array(self.get_prefix())
    }

    #[inline]
    pub fn get_prefix_as_ipv6_addr(&self) -> Ipv6Addr {
        let a = self.get_prefix_as_u16_array();

        Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7])
    }

    #[inline]
    pub fn get_bits(&self) -> u8 {
        mask_to_bits(self.mask).unwrap()
    }

    #[inline]
    /// Get an integer which represents the prefix an IPv6 byte array of this CIDR in big-endian (BE) order.
    pub fn get_mask(&self) -> u128 {
        get_mask(self.get_bits())
    }

    #[inline]
    pub fn get_mask_as_u8_array(&self) -> [u8; 16] {
        self.get_mask().to_be_bytes()
    }

    #[inline]
    pub fn get_mask_as_u16_array(&self) -> [u16; 8] {
        u128_to_u16_array(self.get_mask())
    }

    #[inline]
    pub fn get_mask_as_ipv6_addr(&self) -> Ipv6Addr {
        let a = self.get_mask_as_u16_array();

        Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7])
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn from_prefix_and_bits<P: Ipv6Able>(
        prefix: P,
        bits: u8,
    ) -> Result<Ipv6Cidr, Ipv6CidrError> {
        if bits > 128 {
            return Err(Ipv6CidrError::IncorrectBitsRange);
        }

        let mask = get_mask(bits);

        let prefix = prefix.get_u128() & mask;

        Ok(Ipv6Cidr {
            prefix,
            mask,
        })
    }

    #[inline]
    pub fn from_prefix_and_mask<P: Ipv6Able, M: Ipv6Able>(
        prefix: P,
        mask: M,
    ) -> Result<Ipv6Cidr, Ipv6CidrError> {
        let mask = mask.get_u128();

        match mask_to_bits(mask) {
            Some(_) => {
                let prefix = prefix.get_u128() & mask;

                Ok(Ipv6Cidr {
                    prefix,
                    mask,
                })
            },
            None => Err(Ipv6CidrError::IncorrectMask),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Ipv6Cidr, Ipv6CidrError> {
        let s = s.as_ref();

        match RE_IPV6_CIDR.captures(s) {
            Some(c) => match Ipv6Addr::from_str(c.get(1).unwrap().as_str()) {
                Ok(prefix) => {
                    let bits: u8 =
                        if let Some(m) = c.get(2) { m.as_str().parse().unwrap() } else { 128 };

                    Ipv6Cidr::from_prefix_and_bits(prefix, bits)
                },
                Err(_) => Err(Ipv6CidrError::IncorrectIpv6CIDRString),
            },
            None => Err(Ipv6CidrError::IncorrectIpv6CIDRString),
        }
    }

    #[inline]
    pub fn is_ipv6_cidr<S: AsRef<str>>(s: S) -> bool {
        Self::from_str(s).is_ok()
    }
}

impl Ipv6Cidr {
    #[inline]
    /// Get an integer which represents the first IPv6 byte array of this CIDR in big-endian (BE) order.
    pub fn first(&self) -> u128 {
        self.get_prefix()
    }

    #[inline]
    pub fn first_as_u8_array(&self) -> [u8; 16] {
        self.get_prefix_as_u8_array()
    }

    #[inline]
    pub fn first_as_u16_array(&self) -> [u16; 8] {
        self.get_prefix_as_u16_array()
    }

    #[inline]
    pub fn first_as_ipv6_addr(&self) -> Ipv6Addr {
        self.get_prefix_as_ipv6_addr()
    }

    #[inline]
    /// Get an integer which represents the last IPv6 byte array of this CIDR in big-endian (BE) order.
    pub fn last(&self) -> u128 {
        !self.get_mask() | self.get_prefix()
    }

    #[inline]
    pub fn last_as_u8_array(&self) -> [u8; 16] {
        self.last().to_be_bytes()
    }

    #[inline]
    pub fn last_as_u16_array(&self) -> [u16; 8] {
        u128_to_u16_array(self.last())
    }

    #[inline]
    pub fn last_as_ipv6_addr(&self) -> Ipv6Addr {
        let a = self.last_as_u16_array();

        Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7])
    }

    #[inline]
    pub fn size(&self) -> BigUint {
        BigUint::from(2u8).pow(u32::from(128 - self.get_bits()))
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn contains<IP: Ipv6Able>(&self, ipv6: IP) -> bool {
        let mask = self.get_mask();

        ipv6.get_u128() & mask == self.prefix
    }
}

impl Debug for Ipv6Cidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let prefix = self.get_prefix_as_ipv6_addr();
        let mask = self.get_mask_as_ipv6_addr();
        let bits = self.get_bits();

        debug_helper::impl_debug_for_struct!(Ipv6Cidr, f, self, let .prefix = prefix, let .mask = mask, let .bits = bits);
    }
}

impl Display for Ipv6Cidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let prefix = self.get_prefix_as_ipv6_addr();
        let bits = self.get_bits();

        f.write_fmt(format_args!("{prefix}/{bits}"))
    }
}

impl PartialOrd for Ipv6Cidr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Cidr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ipv6Cidr {
    #[inline]
    fn cmp(&self, other: &Ipv6Cidr) -> Ordering {
        let a = self.first_as_u16_array();
        let b = other.first_as_u16_array();

        for i in 0..8 {
            let cmp_result = a[i].cmp(&b[i]);

            if cmp_result != Ordering::Equal {
                return cmp_result;
            }
        }

        self.get_bits().cmp(&other.get_bits())
    }
}

impl FromStr for Ipv6Cidr {
    type Err = Ipv6CidrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ipv6Cidr::from_str(s)
    }
}

impl TryFrom<&str> for Ipv6Cidr {
    type Error = Ipv6CidrError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ipv6Cidr::from_str(s)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Ipv6Cidr {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Ipv6Cidr {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct Ipv6Visitor;

        impl<'de> Visitor<'de> for Ipv6Visitor {
            type Value = Ipv6Cidr;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                f.write_str("an IPv6 CIDR string")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeError, {
                Ipv6Cidr::from_str(v).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_str(Ipv6Visitor)
    }
}
