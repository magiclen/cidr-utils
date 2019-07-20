use std::net::Ipv4Addr;
use std::mem::transmute;
use std::fmt::{self, Formatter, Display, Debug};

use regex::Regex;
use std::cmp::Ordering;

lazy_static! {
    static ref RE_IPV4_CIDR: Regex = {
        Regex::new(r"^((25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9]))?)?)?)(/((([0-9]|30|31|32)|([1-2][0-9]))|(((25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])))))?$").unwrap()
    };
}

// TODO: Functions

#[inline]
fn get_mask(bits: u8) -> u32 {
    let mut a = [0u8; 4];

    let l = (bits / 8) as usize;

    for i in 0..l {
        a[i] = 255;
    }

    let d = bits % 8;

    if d > 0 {
        a[l] = 0xFF << (8 - d);
    }

    unsafe {
        transmute(a)
    }
}

#[inline]
fn u32_to_u8_array(uint32: u32) -> [u8; 4] {
    unsafe { transmute(uint32) }
}

#[inline]
fn u8_array_to_u32(uint8_array: [u8; 4]) -> u32 {
    unsafe { transmute(uint8_array) }
}

#[inline]
fn mask_to_bits(mask: u32) -> Option<u8> {
    let mut digit = 0;
    let mut b = 32u8;

    for _ in 0..32 {
        let base = (3 - digit / 8) * 8;
        let offset = digit % 8;
        let index = base + offset;

        let n = (mask << index) >> 31;

        digit += 1;

        if n == 0 {
            b = (digit - 1) as u8;
            break;
        }
    }

    for digit in digit..32 {
        let base = (3 - digit / 8) * 8;
        let offset = digit % 8;
        let index = base + offset;

        if mask << index >> 31 == 1 {
            return None;
        }
    }

    Some(b)
}

// TODO: Ipv4Able
/// The type which can be taken as an IPv4 address.
pub trait Ipv4Able {
    #[inline]
    fn get_u32(&self) -> u32;
}

impl Ipv4Able for u32 {
    #[inline]
    fn get_u32(&self) -> u32 {
        *self
    }
}

impl Ipv4Able for [u8; 4] {
    #[inline]
    fn get_u32(&self) -> u32 {
        u8_array_to_u32(*self)
    }
}

impl Ipv4Able for Ipv4Addr {
    #[inline]
    fn get_u32(&self) -> u32 {
        self.octets().get_u32()
    }
}

impl<T: Ipv4Able> Ipv4Able for &T {
    #[inline]
    fn get_u32(&self) -> u32 {
        Ipv4Able::get_u32(*self)
    }
}

// TODO: Ipv4Cidr

/// To represent IPv4 CIDR.
#[derive(PartialEq, Eq, Clone)]
pub struct Ipv4Cidr {
    prefix: u32,
    mask: u32,
}

impl Debug for Ipv4Cidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let prefix = self.get_prefix_as_u8_array();
        let mask = self.get_mask_as_u8_array();
        let bits = self.get_bits();

        impl_debug_for_struct!(Ipv4Cidr, f, self, (.prefix, "{}.{}.{}.{}", prefix[0], prefix[1], prefix[2], prefix[3]), (.mask, "{}.{}.{}.{}", mask[0], mask[1], mask[2], mask[3]), let .bits = bits);
    }
}

impl Display for Ipv4Cidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let prefix = self.get_prefix_as_u8_array();
        let bits = self.get_bits();

        f.write_fmt(format_args!("{}.{}.{}.{}/{}", prefix[0], prefix[1], prefix[2], prefix[3], bits))
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
            if a[i] > b[i] {
                return Ordering::Greater;
            } else if a[i] < b[i] {
                return Ordering::Less;
            }
        }

        self.get_bits().cmp(&other.get_bits())
    }
}

impl Ipv4Cidr {
    #[inline]
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

#[derive(Debug, PartialEq)]
/// Possible errors of `Ipv4Cidr`.
pub enum Ipv4CidrError {
    IncorrectBitsRange,
    IncorrectMask,
    IncorrectIpv4CIDRString,
}

impl Ipv4Cidr {
    pub fn from_prefix_and_bits<P: Ipv4Able>(prefix: P, bits: u8) -> Result<Ipv4Cidr, Ipv4CidrError> {
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

    pub fn from_prefix_and_mask<P: Ipv4Able, M: Ipv4Able>(prefix: P, mask: M) -> Result<Ipv4Cidr, Ipv4CidrError> {
        let mask = mask.get_u32();

        match mask_to_bits(mask) {
            Some(_) => {
                let prefix = prefix.get_u32() & mask;

                Ok(Ipv4Cidr {
                    prefix,
                    mask,
                })
            }
            None => {
                Err(Ipv4CidrError::IncorrectMask)
            }
        }
    }

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

                if let Some(_) = c.get(13) {
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
                            None => {
                                Err(Ipv4CidrError::IncorrectIpv4CIDRString)
                            }
                        }
                    }
                } else {
                    Ipv4Cidr::from_prefix_and_bits(prefix, prefer_bits.unwrap_or(32))
                }
            }
            None => {
                Err(Ipv4CidrError::IncorrectIpv4CIDRString)
            }
        }
    }

    pub fn is_ipv4_cidr<S: AsRef<str>>(s: S) -> bool {
        Self::from_str(s).is_ok()
    }
}

impl Ipv4Cidr {
    #[inline]
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
        2u64.pow((32 - self.get_bits()) as u32)
    }
}

impl Ipv4Cidr {
    #[inline]
    pub fn contains<IP: Ipv4Able>(&self, ipv4: IP) -> bool {
        let mask = self.get_mask();

        ipv4.get_u32() & mask == self.prefix
    }
}

// TODO: Ipv4CidrU8ArrayIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrU8ArrayIterator {
    rev_from: u32,
    next: u64,
    size: u64,
}

impl Iterator for Ipv4CidrU8ArrayIterator {
    type Item = [u8; 4];

    #[inline]
    fn next(&mut self) -> Option<[u8; 4]> {
        if self.next == self.size {
            None
        } else {
            let p = self.rev_from + self.next as u32;

            self.next += 1;

            let a = u32_to_u8_array(p);

            Some([a[3], a[2], a[1], a[0]])
        }
    }

    #[inline]
    fn last(mut self) -> Option<[u8; 4]> {
        self.next = self.size - 1;

        self.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<[u8; 4]> {
        self.nth_u64(n as u64)
    }
}

impl Ipv4CidrU8ArrayIterator {
    #[inline]
    pub fn nth_u64(&mut self, n: u64) -> Option<[u8; 4]> {
        let n_u64 = n.min(self.size - self.next);

        self.next += n_u64;

        self.next()
    }
}

impl Ipv4Cidr {
    #[inline]
    pub fn iter_as_u8_array(&self) -> Ipv4CidrU8ArrayIterator {
        let a = self.get_prefix_as_u8_array();

        let rev_from = u8_array_to_u32([a[3], a[2], a[1], a[0]]);

        Ipv4CidrU8ArrayIterator {
            rev_from,
            next: 0,
            size: self.size(),
        }
    }
}

// TODO: Ipv4CidrIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrIterator {
    iter: Ipv4CidrU8ArrayIterator
}

impl Iterator for Ipv4CidrIterator {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<u32> {
        self.iter.next().map(|a| u8_array_to_u32(a))
    }

    #[inline]
    fn last(self) -> Option<u32> {
        self.iter.last().map(|a| u8_array_to_u32(a))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<u32> {
        self.iter.nth(n).map(|a| u8_array_to_u32(a))
    }
}

impl Ipv4CidrIterator {
    #[inline]
    pub fn nth_u64(&mut self, n: u64) -> Option<u32> {
        self.iter.nth_u64(n).map(|a| u8_array_to_u32(a))
    }
}

impl Ipv4Cidr {
    #[inline]
    pub fn iter(&self) -> Ipv4CidrIterator {
        let iter = self.iter_as_u8_array();

        Ipv4CidrIterator {
            iter
        }
    }
}

// TODO: Ipv4CidrIpv4AddrIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrIpv4AddrIterator {
    iter: Ipv4CidrU8ArrayIterator
}

impl Iterator for Ipv4CidrIpv4AddrIterator {
    type Item = Ipv4Addr;

    #[inline]
    fn next(&mut self) -> Option<Ipv4Addr> {
        self.iter.next().map(|a| Ipv4Addr::new(a[0], a[1], a[2], a[3]))
    }

    #[inline]
    fn last(self) -> Option<Ipv4Addr> {
        self.iter.last().map(|a| Ipv4Addr::new(a[0], a[1], a[2], a[3]))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Ipv4Addr> {
        self.iter.nth(n).map(|a| Ipv4Addr::new(a[0], a[1], a[2], a[3]))
    }
}

impl Ipv4CidrIpv4AddrIterator {
    #[inline]
    pub fn nth_u64(&mut self, n: u64) -> Option<Ipv4Addr> {
        self.iter.nth_u64(n).map(|a| Ipv4Addr::new(a[0], a[1], a[2], a[3]))
    }
}

impl Ipv4Cidr {
    #[inline]
    pub fn iter_as_ipv4_addr(&self) -> Ipv4CidrIpv4AddrIterator {
        let iter = self.iter_as_u8_array();

        Ipv4CidrIpv4AddrIterator {
            iter
        }
    }
}