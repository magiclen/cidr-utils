use std::fmt::{self, Debug, Display, Formatter};
use std::mem::transmute;
use std::net::Ipv6Addr;
use std::str::FromStr;

use regex::Regex;
use std::cmp::Ordering;

lazy_static! {
    static ref RE_IPV6_CIDR: Regex = {
        Regex::new(r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))(/((12[0-8])|(1[0-1][0-9])|([1-9][0-9])|[0-9]))?$").unwrap()
    };
}

// TODO: Functions

#[inline]
fn subtract(a: (u128, bool), b: (u128, bool)) -> (u128, bool) {
    if a.1 {
        if b.1 {
            (0, false)
        } else if b.0 == 0 {
            (0, true)
        } else {
            (u128::max_value() - b.0 + 1, false)
        }
    } else if b.1 {
        unreachable!()
    } else {
        (a.0 - b.0, false)
    }
}

#[inline]
fn get_mask(bits: u8) -> u128 {
    let mut a = [0u8; 16];

    let l = (bits / 8) as usize;

    for e in a.iter_mut().take(l) {
        *e = 255;
    }

    let d = bits % 8;

    if d > 0 {
        a[l] = 0xFF << (8 - d);
    }

    unsafe { transmute(a) }
}

#[inline]
fn u128_to_u8_array(uint128: u128) -> [u8; 16] {
    unsafe { transmute(uint128) }
}

#[inline]
fn u128_to_u16_array(uint128: u128) -> [u16; 8] {
    let a = u128_to_u8_array(uint128);

    unsafe {
        transmute([
            a[1], a[0], a[3], a[2], a[5], a[4], a[7], a[6], a[9], a[8], a[11], a[10], a[13], a[12],
            a[15], a[14],
        ])
    }
}

#[inline]
fn u8_array_to_u128(uint8_array: [u8; 16]) -> u128 {
    unsafe { transmute(uint8_array) }
}

#[inline]
fn u16_array_to_u128(uint8_array: [u16; 8]) -> u128 {
    let a: [u8; 16] = unsafe { transmute(uint8_array) };

    unsafe {
        transmute([
            a[1], a[0], a[3], a[2], a[5], a[4], a[7], a[6], a[9], a[8], a[11], a[10], a[13], a[12],
            a[15], a[14],
        ])
    }
}

#[inline]
fn mask_to_bits(mask: u128) -> Option<u8> {
    let mut digit = 0;
    let mut b = 128u8;

    for _ in 0..128 {
        let base = (15 - digit / 8) * 8;
        let offset = digit % 8;
        let index = base + offset;

        let n = (mask << index) >> 127;

        digit += 1;

        if n == 0 {
            b = (digit - 1) as u8;
            break;
        }
    }

    for digit in digit..128 {
        let base = (15 - digit / 8) * 8;
        let offset = digit % 8;
        let index = base + offset;

        if mask << index >> 127 == 1 {
            return None;
        }
    }

    Some(b)
}

// TODO: Ipv6Able
/// The type which can be taken as an IPv6 address.
pub trait Ipv6Able {
    fn get_u128(&self) -> u128;
}

impl Ipv6Able for u128 {
    #[inline]
    fn get_u128(&self) -> u128 {
        *self
    }
}

impl Ipv6Able for [u8; 16] {
    #[inline]
    fn get_u128(&self) -> u128 {
        u8_array_to_u128(*self)
    }
}

impl Ipv6Able for [u16; 8] {
    #[inline]
    fn get_u128(&self) -> u128 {
        u16_array_to_u128(*self)
    }
}

impl Ipv6Able for Ipv6Addr {
    #[inline]
    fn get_u128(&self) -> u128 {
        self.segments().get_u128()
    }
}

impl<T: Ipv6Able> Ipv6Able for &T {
    #[inline]
    fn get_u128(&self) -> u128 {
        Ipv6Able::get_u128(*self)
    }
}

// TODO: Ipv6Cidr

/// To represent IPv6 CIDR.
#[derive(PartialEq, Eq, Clone)]
pub struct Ipv6Cidr {
    prefix: u128,
    mask: u128,
}

impl Debug for Ipv6Cidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let prefix = self.get_prefix_as_u16_array();
        let mask = self.get_mask_as_u16_array();
        let bits = self.get_bits();

        impl_debug_for_struct!(Ipv6Cidr, f, self, (.prefix, "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}", prefix[0], prefix[1], prefix[2], prefix[3], prefix[4], prefix[5], prefix[6], prefix[7]), (.mask, "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}", mask[0], mask[1], mask[2], mask[3], mask[4], mask[5], mask[6], mask[7]), let .bits = bits);
    }
}

impl Display for Ipv6Cidr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let prefix = self.get_prefix_as_u16_array();
        let bits = self.get_bits();

        f.write_fmt(format_args!(
            "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}/{}",
            prefix[0],
            prefix[1],
            prefix[2],
            prefix[3],
            prefix[4],
            prefix[5],
            prefix[6],
            prefix[7],
            bits
        ))
    }
}

impl PartialOrd for Ipv6Cidr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Cidr) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Ipv6Cidr {
    #[inline]
    fn cmp(&self, other: &Ipv6Cidr) -> Ordering {
        let a = self.first_as_u16_array();
        let b = other.first_as_u16_array();

        for i in 0..16 {
            if a[i] > b[i] {
                return Ordering::Greater;
            } else if a[i] < b[i] {
                return Ordering::Less;
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

impl Ipv6Cidr {
    #[inline]
    pub fn get_prefix(&self) -> u128 {
        self.prefix
    }

    #[inline]
    pub fn get_prefix_as_u8_array(&self) -> [u8; 16] {
        u128_to_u8_array(self.get_prefix())
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
    pub fn get_mask(&self) -> u128 {
        get_mask(self.get_bits())
    }

    #[inline]
    pub fn get_mask_as_u8_array(&self) -> [u8; 16] {
        u128_to_u8_array(self.get_mask())
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

#[derive(Debug, PartialEq)]
/// Possible errors of `Ipv6Cidr`.
pub enum Ipv6CidrError {
    IncorrectBitsRange,
    IncorrectMask,
    IncorrectIpv6CIDRString,
}

impl Ipv6Cidr {
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
            }
            None => Err(Ipv6CidrError::IncorrectMask),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Ipv6Cidr, Ipv6CidrError> {
        let s = s.as_ref();

        match RE_IPV6_CIDR.captures(s) {
            Some(c) => {
                let prefix = Ipv6Addr::from_str(c.get(1).unwrap().as_str()).unwrap().segments();

                let bits: u8 = if let Some(m) = c.get(32) {
                    m.as_str().parse().unwrap()
                } else {
                    128
                };

                Ipv6Cidr::from_prefix_and_bits(prefix, bits)
            }
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
    pub fn last(&self) -> u128 {
        !self.get_mask() | self.get_prefix()
    }

    #[inline]
    pub fn last_as_u8_array(&self) -> [u8; 16] {
        u128_to_u8_array(self.last())
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
    pub fn size(&self) -> (u128, bool) {
        let bits = self.get_bits();

        if bits == 0 {
            (0, true)
        } else {
            (2u128.pow(u32::from(128 - self.get_bits())), false)
        }
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn contains<IP: Ipv6Able>(&self, ipv6: IP) -> bool {
        let mask = self.get_mask();

        ipv6.get_u128() & mask == self.prefix
    }
}

// TODO: Ipv6CidrU8ArrayIterator

/// To iterate IPv6 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrU8ArrayIterator {
    rev_from: u128,
    next: (u128, bool),
    size: (u128, bool),
}

impl Iterator for Ipv6CidrU8ArrayIterator {
    type Item = [u8; 16];

    #[inline]
    fn next(&mut self) -> Option<[u8; 16]> {
        if self.next == self.size {
            None
        } else {
            let p = self.rev_from + self.next.0;

            if self.next.0 == u128::max_value() {
                self.next = (0, true);
            } else {
                self.next.0 += 1;
            }

            let a = u128_to_u8_array(p);

            Some([
                a[15], a[14], a[13], a[12], a[11], a[10], a[9], a[8], a[7], a[6], a[5], a[4], a[3],
                a[2], a[1], a[0],
            ])
        }
    }

    #[inline]
    fn last(mut self) -> Option<[u8; 16]> {
        self.next = (self.size.0 - 1, self.size.1);

        self.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<[u8; 16]> {
        self.nth_u128((n as u128, false))
    }
}

impl Ipv6CidrU8ArrayIterator {
    #[inline]
    pub fn nth_u128(&mut self, n: (u128, bool)) -> Option<[u8; 16]> {
        if n.1 {
            self.next = self.size;
        } else {
            let d = subtract(self.size, self.next);

            if d.1 {
                self.next.0 += n.0;
            } else {
                let n = n.0.min(d.0);

                if u128::max_value() - n < self.next.0 {
                    self.next = self.size;
                } else {
                    self.next.0 += n;
                }
            }
        }

        self.next()
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn iter_as_u8_array(&self) -> Ipv6CidrU8ArrayIterator {
        let a = self.get_prefix_as_u8_array();

        let rev_from = u8_array_to_u128([
            a[15], a[14], a[13], a[12], a[11], a[10], a[9], a[8], a[7], a[6], a[5], a[4], a[3],
            a[2], a[1], a[0],
        ]);

        Ipv6CidrU8ArrayIterator {
            rev_from,
            next: (0, false),
            size: self.size(),
        }
    }
}

// TODO: Ipv6CidrU8ArrayIterator

/// To iterate IPv6 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrU16ArrayIterator {
    rev_from: u128,
    next: (u128, bool),
    size: (u128, bool),
}

impl Iterator for Ipv6CidrU16ArrayIterator {
    type Item = [u16; 8];

    #[inline]
    fn next(&mut self) -> Option<[u16; 8]> {
        if self.next == self.size {
            None
        } else {
            let p = self.rev_from + self.next.0;

            if self.next.0 == u128::max_value() {
                self.next = (0, true);
            } else {
                self.next.0 += 1;
            }

            let a = u128_to_u8_array(p);

            Some(u128_to_u16_array(u8_array_to_u128([
                a[15], a[14], a[13], a[12], a[11], a[10], a[9], a[8], a[7], a[6], a[5], a[4], a[3],
                a[2], a[1], a[0],
            ])))
        }
    }

    #[inline]
    fn last(mut self) -> Option<[u16; 8]> {
        self.next = (self.size.0 - 1, self.size.1);

        self.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<[u16; 8]> {
        self.nth_u128((n as u128, false))
    }
}

impl Ipv6CidrU16ArrayIterator {
    #[inline]
    pub fn nth_u128(&mut self, n: (u128, bool)) -> Option<[u16; 8]> {
        if n.1 {
            self.next = self.size;
        } else {
            let d = subtract(self.size, self.next);

            if d.1 {
                self.next.0 += n.0;
            } else {
                let n = n.0.min(d.0);

                self.next.0 += n;
            }
        }

        self.next()
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn iter_as_u16_array(&self) -> Ipv6CidrU16ArrayIterator {
        let a = self.get_prefix_as_u8_array();

        let rev_from = u8_array_to_u128([
            a[15], a[14], a[13], a[12], a[11], a[10], a[9], a[8], a[7], a[6], a[5], a[4], a[3],
            a[2], a[1], a[0],
        ]);

        Ipv6CidrU16ArrayIterator {
            rev_from,
            next: (0, false),
            size: self.size(),
        }
    }
}

// TODO: Ipv6CidrIterator

/// To iterate IPv6 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrIterator {
    iter: Ipv6CidrU8ArrayIterator,
}

impl Iterator for Ipv6CidrIterator {
    type Item = u128;

    #[inline]
    fn next(&mut self) -> Option<u128> {
        self.iter.next().map(u8_array_to_u128)
    }

    #[inline]
    fn last(self) -> Option<u128> {
        self.iter.last().map(u8_array_to_u128)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<u128> {
        self.iter.nth(n).map(u8_array_to_u128)
    }
}

impl Ipv6CidrIterator {
    #[inline]
    pub fn nth_u128(&mut self, n: (u128, bool)) -> Option<u128> {
        self.iter.nth_u128(n).map(u8_array_to_u128)
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn iter(&self) -> Ipv6CidrIterator {
        let iter = self.iter_as_u8_array();

        Ipv6CidrIterator {
            iter,
        }
    }
}

// TODO: Ipv6CidrIpv6AddrIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrIpv6AddrIterator {
    iter: Ipv6CidrU16ArrayIterator,
}

impl Iterator for Ipv6CidrIpv6AddrIterator {
    type Item = Ipv6Addr;

    #[inline]
    fn next(&mut self) -> Option<Ipv6Addr> {
        self.iter.next().map(|a| Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]))
    }

    #[inline]
    fn last(self) -> Option<Ipv6Addr> {
        self.iter.last().map(|a| Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Ipv6Addr> {
        self.iter.nth(n).map(|a| Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]))
    }
}

impl Ipv6CidrIpv6AddrIterator {
    #[inline]
    pub fn nth_u128(&mut self, n: (u128, bool)) -> Option<Ipv6Addr> {
        self.iter.nth_u128(n).map(|a| Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]))
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn iter_as_ipv6_addr(&self) -> Ipv6CidrIpv6AddrIterator {
        let iter = self.iter_as_u16_array();

        Ipv6CidrIpv6AddrIterator {
            iter,
        }
    }
}
