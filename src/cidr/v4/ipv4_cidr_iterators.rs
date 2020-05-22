use std::net::Ipv4Addr;

use super::functions::*;
use super::Ipv4Cidr;

// TODO: Ipv4CidrU8ArrayIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrU8ArrayIterator {
    from: u32,
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
            let p = self.from + self.next as u32;

            self.next += 1;

            Some(u32_to_u8_array(p))
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
        let from = self.first();

        Ipv4CidrU8ArrayIterator {
            from,
            next: 0,
            size: self.size(),
        }
    }
}

// TODO: Ipv4CidrIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrIterator {
    iter: Ipv4CidrU8ArrayIterator,
}

impl Iterator for Ipv4CidrIterator {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<u32> {
        self.iter.next().map(u8_array_to_u32)
    }

    #[inline]
    fn last(self) -> Option<u32> {
        self.iter.last().map(u8_array_to_u32)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<u32> {
        self.iter.nth(n).map(u8_array_to_u32)
    }
}

impl Ipv4CidrIterator {
    #[inline]
    pub fn nth_u64(&mut self, n: u64) -> Option<u32> {
        self.iter.nth_u64(n).map(u8_array_to_u32)
    }
}

impl Ipv4Cidr {
    #[inline]
    pub fn iter(&self) -> Ipv4CidrIterator {
        let iter = self.iter_as_u8_array();

        Ipv4CidrIterator {
            iter,
        }
    }
}

// TODO: Ipv4CidrIpv4AddrIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrIpv4AddrIterator {
    iter: Ipv4CidrU8ArrayIterator,
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
            iter,
        }
    }
}
