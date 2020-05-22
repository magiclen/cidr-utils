use std::net::Ipv6Addr;

use super::functions::*;
use super::Ipv6Cidr;

// TODO: Ipv6CidrU8ArrayIterator

/// To iterate IPv6 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrU8ArrayIterator {
    from: u128,
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
            let p = self.from + self.next.0;

            if self.next.0 == u128::max_value() {
                self.next = (0, true);
            } else {
                self.next.0 += 1;
            }

            Some(u128_to_u8_array(p))
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
        let from = self.first();

        Ipv6CidrU8ArrayIterator {
            from,
            next: (0, false),
            size: self.size(),
        }
    }
}

// TODO: Ipv6CidrU8ArrayIterator

/// To iterate IPv6 CIDRs.
#[derive(Debug)]
pub struct Ipv6CidrU16ArrayIterator {
    from: u128,
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
            let p = self.from + self.next.0;

            if self.next.0 == u128::max_value() {
                self.next = (0, true);
            } else {
                self.next.0 += 1;
            }

            Some(u128_to_u16_array(p))
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
        let from = self.first();

        Ipv6CidrU16ArrayIterator {
            from,
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
