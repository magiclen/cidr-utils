use std::net::Ipv6Addr;

use num_traits::{One, ToPrimitive, Zero};

use super::{functions::*, Ipv6Cidr};
use crate::num_bigint::BigUint;

// TODO: Ipv6CidrU8ArrayIterator

/// To iterate IPv6 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrU8ArrayIterator {
    from: u128,
    next: BigUint,
    back: BigUint,
    size: BigUint,
}

impl Ipv6CidrU8ArrayIterator {
    #[inline]
    unsafe fn next_unchecked(&mut self) -> [u8; 16] {
        let p = self.from + self.next.to_u128().unwrap();

        self.next += BigUint::one();

        p.to_be_bytes()
    }

    #[inline]
    unsafe fn next_back_unchecked(&mut self) -> [u8; 16] {
        self.back -= BigUint::one();

        let p = self.from + self.back.to_u128().unwrap();

        p.to_be_bytes()
    }

    #[inline]
    pub fn nth_big_uint(&mut self, n: BigUint) -> Option<[u8; 16]> {
        self.next += n;

        if self.next < self.back {
            Some(unsafe { self.next_unchecked() })
        } else {
            self.next = self.size.clone();

            None
        }
    }

    #[inline]
    pub fn nth_back_big_uint(&mut self, n: BigUint) -> Option<[u8; 16]> {
        if self.back > n {
            self.back -= n;

            if self.next < self.back {
                return Some(unsafe { self.next_back_unchecked() });
            }
        }

        self.next = self.size.clone();

        None
    }
}

impl Iterator for Ipv6CidrU8ArrayIterator {
    type Item = [u8; 16];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.back {
            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        if self.next < self.back {
            self.next = self.back.clone() - BigUint::one();

            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.nth_big_uint(BigUint::from(n))
    }
}

impl DoubleEndedIterator for Ipv6CidrU8ArrayIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.next < self.back {
            Some(unsafe { self.next_back_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.nth_back_big_uint(BigUint::from(n))
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn iter_as_u8_array(&self) -> Ipv6CidrU8ArrayIterator {
        let from = self.first();
        let size = self.size();

        Ipv6CidrU8ArrayIterator {
            from,
            next: BigUint::zero(),
            back: size.clone(),
            size,
        }
    }
}

// TODO: Ipv6CidrU8ArrayIterator

/// To iterate IPv6 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrU16ArrayIterator {
    from: u128,
    next: BigUint,
    back: BigUint,
    size: BigUint,
}

impl Ipv6CidrU16ArrayIterator {
    #[inline]
    unsafe fn next_unchecked(&mut self) -> [u16; 8] {
        let p = self.from + self.next.to_u128().unwrap();

        self.next += BigUint::one();

        u128_to_u16_array(p)
    }

    #[inline]
    unsafe fn next_back_unchecked(&mut self) -> [u16; 8] {
        self.back -= BigUint::one();

        let p = self.from + self.back.to_u128().unwrap();

        u128_to_u16_array(p)
    }

    #[inline]
    pub fn nth_big_uint(&mut self, n: BigUint) -> Option<[u16; 8]> {
        self.next += n;

        if self.next < self.back {
            Some(unsafe { self.next_unchecked() })
        } else {
            self.next = self.size.clone();

            None
        }
    }

    #[inline]
    pub fn nth_back_big_uint(&mut self, n: BigUint) -> Option<[u16; 8]> {
        if self.back > n {
            self.back -= n;

            if self.next < self.back {
                return Some(unsafe { self.next_back_unchecked() });
            }
        }

        self.next = self.size.clone();

        None
    }
}

impl Iterator for Ipv6CidrU16ArrayIterator {
    type Item = [u16; 8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.back {
            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        if self.next < self.back {
            self.next = self.back.clone() - BigUint::one();

            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.nth_big_uint(BigUint::from(n))
    }
}

impl DoubleEndedIterator for Ipv6CidrU16ArrayIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.next < self.back {
            Some(unsafe { self.next_back_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.nth_back_big_uint(BigUint::from(n))
    }
}

impl Ipv6Cidr {
    #[inline]
    pub fn iter_as_u16_array(&self) -> Ipv6CidrU16ArrayIterator {
        let from = self.first();
        let size = self.size();

        Ipv6CidrU16ArrayIterator {
            from,
            next: BigUint::zero(),
            back: size.clone(),
            size,
        }
    }
}

// TODO: Ipv6CidrIterator

/// To iterate IPv6 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrIterator {
    iter: Ipv6CidrU8ArrayIterator,
}

impl Ipv6CidrIterator {
    #[inline]
    pub fn nth_big_uint(&mut self, n: BigUint) -> Option<u128> {
        self.iter.nth_big_uint(n).map(u128::from_be_bytes)
    }

    #[inline]
    pub fn nth_back_big_uint(&mut self, n: BigUint) -> Option<u128> {
        self.iter.nth_back_big_uint(n).map(u128::from_be_bytes)
    }
}

impl Iterator for Ipv6CidrIterator {
    type Item = u128;

    #[inline]
    fn next(&mut self) -> Option<u128> {
        self.iter.next().map(u128::from_be_bytes)
    }

    #[inline]
    fn last(self) -> Option<u128> {
        self.iter.last().map(u128::from_be_bytes)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<u128> {
        self.iter.nth(n).map(u128::from_be_bytes)
    }
}

impl DoubleEndedIterator for Ipv6CidrIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(u128::from_be_bytes)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(u128::from_be_bytes)
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

impl Ipv6CidrIpv6AddrIterator {
    #[inline]
    pub fn nth_big_int(&mut self, n: BigUint) -> Option<Ipv6Addr> {
        self.iter
            .nth_big_uint(n)
            .map(|a| Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]))
    }

    #[inline]
    pub fn nth_back_big_int(&mut self, n: BigUint) -> Option<Ipv6Addr> {
        self.iter
            .nth_back_big_uint(n)
            .map(|a| Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]))
    }
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

impl DoubleEndedIterator for Ipv6CidrIpv6AddrIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|a| Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|a| Ipv6Addr::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]))
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
