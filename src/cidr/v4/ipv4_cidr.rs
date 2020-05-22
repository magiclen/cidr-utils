extern crate debug_helper;
extern crate lazy_static;
extern crate regex;

use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::net::Ipv4Addr;
use std::str::FromStr;

use super::functions::*;
use super::{Ipv4Able, Ipv4CidrError};

use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_IPV4_CIDR: Regex = {
        Regex::new(r"^((25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9]))?)?)?)(/((([0-9]|30|31|32)|([1-2][0-9]))|(((25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])))))?$").unwrap()
    };
}

/// To represent IPv4 CIDR.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ipv4Cidr {
    prefix: u32,
    mask: u32,
}

impl Ipv4Cidr {
    #[inline]
    /// Get an integer which represents the prefix an IPv4 byte array of this CIDR in big-endian (BE) order.
    pub fn get_prefix(&self) -> u32 {
        self.prefix
    }

    #[inline]
    pub fn get_prefix_as_u8_array(&self) -> [u8; 4] {
        u32_to_u8_array(self.get_prefix())
    }

    #[inline]
    pub fn get_prefix_as_ipv4_addr(&self) -> Ipv4Addr {
        let a = self.get_prefix_as_u8_array();

        Ipv4Addr::new(a[0], a[1], a[2], a[3])
    }

    #[inline]
    pub fn get_bits(&self) -> u8 {
        mask_to_bits(self.mask).unwrap()
    }

    #[inline]
    /// Get an integer which represents the mask an IPv4 byte array of this CIDR in big-endian (BE) order.
    pub fn get_mask(&self) -> u32 {
        get_mask(self.get_bits())
    }

    #[inline]
    pub fn get_mask_as_u8_array(&self) -> [u8; 4] {
        u32_to_u8_array(self.get_mask())
    }

    #[inline]
    pub fn get_mask_as_ipv4_addr(&self) -> Ipv4Addr {
        let a = self.get_mask_as_u8_array();

        Ipv4Addr::new(a[0], a[1], a[2], a[3])
    }
}

impl Ipv4Cidr {
    pub fn from_prefix_and_bits<P: Ipv4Able>(
        prefix: P,
        bits: u8,
    ) -> Result<Ipv4Cidr, Ipv4CidrError> {
        if bits > 32 {
            return Err(Ipv4CidrError::IncorrectBitsRange);
        }

        let mask = get_mask(bits);

        let prefix = prefix.get_u32() & mask;

        Ok(Ipv4Cidr {
            prefix,
            mask,
        })
    }

    pub fn from_prefix_and_mask<P: Ipv4Able, M: Ipv4Able>(
        prefix: P,
        mask: M,
    ) -> Result<Ipv4Cidr, Ipv4CidrError> {
        let mask = mask.get_u32();

        match mask_to_bits(mask) {
            Some(_) => {
                let prefix = prefix.get_u32() & mask;

                Ok(Ipv4Cidr {
                    prefix,
                    mask,
                })
            }
            None => Err(Ipv4CidrError::IncorrectMask),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Ipv4Cidr, Ipv4CidrError> {
        let s = s.as_ref();

        match RE_IPV4_CIDR.captures(s) {
            Some(c) => {
                let mut prefix = [0u8; 4];
                let mut prefer_bits = None;

                prefix[0] = c.get(2).unwrap().as_str().parse().unwrap();
                match c.get(5).map(|m| m.as_str().parse().unwrap()) {
                    Some(n) => {
                        prefix[1] = n;

                        match c.get(8).map(|m| m.as_str().parse().unwrap()) {
                            Some(n) => {
                                prefix[2] = n;

                                match c.get(11).map(|m| m.as_str().parse().unwrap()) {
                                    Some(n) => {
                                        prefix[3] = n;
                                    }
                                    None => {
                                        prefer_bits = Some(24);
                                    }
                                }
                            }
                            None => {
                                prefer_bits = Some(16);
                            }
                        }
                    }
                    None => {
                        prefer_bits = Some(8);
                    }
                }

                if c.get(13).is_some() {
                    if let Some(m) = c.get(15) {
                        let bits = m.as_str().parse().unwrap();

                        if let Some(prefer_bits) = prefer_bits {
                            if bits != prefer_bits {
                                return Err(Ipv4CidrError::IncorrectIpv4CIDRString);
                            }
                        }

                        Ok(Ipv4Cidr::from_prefix_and_bits(prefix, bits)?)
                    } else {
                        let mut mask = [0u8; 4];

                        mask[0] = c.get(20).unwrap().as_str().parse().unwrap();
                        mask[1] = c.get(22).unwrap().as_str().parse().unwrap();
                        mask[2] = c.get(24).unwrap().as_str().parse().unwrap();
                        mask[3] = c.get(26).unwrap().as_str().parse().unwrap();

                        match mask_to_bits(u8_array_to_u32(mask)) {
                            Some(bits) => {
                                if let Some(prefer_bits) = prefer_bits {
                                    if bits != prefer_bits {
                                        return Err(Ipv4CidrError::IncorrectIpv4CIDRString);
                                    }
                                }

                                Ipv4Cidr::from_prefix_and_mask(prefix, mask)
                            }
                            None => Err(Ipv4CidrError::IncorrectIpv4CIDRString),
                        }
                    }
                } else {
                    Ipv4Cidr::from_prefix_and_bits(prefix, prefer_bits.unwrap_or(32))
                }
            }
            None => Err(Ipv4CidrError::IncorrectIpv4CIDRString),
        }
    }

    pub fn is_ipv4_cidr<S: AsRef<str>>(s: S) -> bool {
        Self::from_str(s).is_ok()
    }
}

impl Ipv4Cidr {
    #[inline]
    /// Get an integer which represents the first IPv4 byte array of this CIDR in big-endian (BE) order.
    pub fn first(&self) -> u32 {
        self.get_prefix()
    }

    #[inline]
    pub fn first_as_u8_array(&self) -> [u8; 4] {
        self.get_prefix_as_u8_array()
    }

    #[inline]
    pub fn first_as_ipv4_addr(&self) -> Ipv4Addr {
        self.get_prefix_as_ipv4_addr()
    }

    #[inline]
    /// Get an integer which represents the last IPv4 byte array of this CIDR in big-endian (BE) order.
    pub fn last(&self) -> u32 {
        !self.get_mask() | self.get_prefix()
    }

    #[inline]
    pub fn last_as_u8_array(&self) -> [u8; 4] {
        u32_to_u8_array(self.last())
    }

    #[inline]
    pub fn last_as_ipv4_addr(&self) -> Ipv4Addr {
        let a = self.last_as_u8_array();

        Ipv4Addr::new(a[0], a[1], a[2], a[3])
    }

    #[inline]
    pub fn size(&self) -> u64 {
        2u64.pow(u32::from(32 - self.get_bits()))
    }
}

impl Ipv4Cidr {
    #[inline]
    pub fn contains<IP: Ipv4Able>(&self, ipv4: IP) -> bool {
        let mask = self.get_mask();

        ipv4.get_u32() & mask == self.prefix
    }
}

impl Debug for Ipv4Cidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let prefix = self.get_prefix_as_u8_array();
        let mask = self.get_mask_as_u8_array();
        let bits = self.get_bits();

        debug_helper::impl_debug_for_struct!(Ipv4Cidr, f, self, (.prefix, "{}.{}.{}.{}", prefix[0], prefix[1], prefix[2], prefix[3]), (.mask, "{}.{}.{}.{}", mask[0], mask[1], mask[2], mask[3]), let .bits = bits);
    }
}

impl Display for Ipv4Cidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let prefix = self.get_prefix_as_u8_array();
        let bits = self.get_bits();

        f.write_fmt(format_args!(
            "{}.{}.{}.{}/{}",
            prefix[0], prefix[1], prefix[2], prefix[3], bits
        ))
    }
}

impl PartialOrd for Ipv4Cidr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Cidr) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Ipv4Cidr {
    #[inline]
    fn cmp(&self, other: &Ipv4Cidr) -> Ordering {
        let a = self.first_as_u8_array();
        let b = other.first_as_u8_array();

        for i in 0..4 {
            let cmp_result = a[i].cmp(&b[i]);

            if cmp_result != Ordering::Equal {
                return cmp_result;
            }
        }

        self.get_bits().cmp(&other.get_bits())
    }
}

impl FromStr for Ipv4Cidr {
    type Err = Ipv4CidrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ipv4Cidr::from_str(s)
    }
}
