use std::net::Ipv4Addr;

use cidr::Ipv4Cidr;

use crate::Ipv4CidrSize;

// TODO: Ipv4CidrU8ArrayIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrU8ArrayIterator {
    from: u32,
    size: u64,
    next: u64,
    back: u64,
}

impl Ipv4CidrU8ArrayIterator {
    #[inline]
    pub fn new(cidr: &Ipv4Cidr) -> Self {
        let from: u32 = cidr.first_address().into();
        let size = cidr.size();

        Self {
            from,
            size,
            next: 0,
            back: size,
        }
    }
}

impl Ipv4CidrU8ArrayIterator {
    #[inline]
    unsafe fn next_unchecked(&mut self) -> [u8; 4] {
        let p = self.from + self.next as u32;

        self.next += 1;

        p.to_be_bytes()
    }

    #[inline]
    unsafe fn next_back_unchecked(&mut self) -> [u8; 4] {
        self.back -= 1;

        let p = self.from + self.back as u32;

        p.to_be_bytes()
    }

    #[inline]
    pub fn nth_u64(&mut self, n: u64) -> Option<[u8; 4]> {
        self.next += n;

        if self.next < self.back {
            Some(unsafe { self.next_unchecked() })
        } else {
            self.next = self.size;

            None
        }
    }

    #[inline]
    pub fn nth_back_u64(&mut self, n: u64) -> Option<[u8; 4]> {
        if self.back > n {
            self.back -= n;

            if self.next < self.back {
                return Some(unsafe { self.next_back_unchecked() });
            }
        }

        self.next = self.size;

        None
    }
}

impl Iterator for Ipv4CidrU8ArrayIterator {
    type Item = [u8; 4];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.back {
            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[cfg(not(any(
        target_pointer_width = "8",
        target_pointer_width = "16",
        target_pointer_width = "32"
    )))]
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_ips = (self.back - self.next) as usize;

        (remaining_ips, Some(remaining_ips))
    }

    #[cfg(not(any(
        target_pointer_width = "8",
        target_pointer_width = "16",
        target_pointer_width = "32"
    )))]
    #[inline]
    fn count(self) -> usize
    where
        Self: Sized, {
        if self.next < self.back {
            (self.back - self.next) as usize
        } else {
            0
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        if self.next < self.back {
            self.next = self.back - 1;

            Some(unsafe { self.next_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.nth_u64(n as u64)
    }
}

impl DoubleEndedIterator for Ipv4CidrU8ArrayIterator {
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
        self.nth_back_u64(n as u64)
    }
}

// TODO: Ipv4CidrIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrIterator {
    iter: Ipv4CidrU8ArrayIterator,
}

impl Ipv4CidrIterator {
    #[inline]
    pub fn new(cidr: &Ipv4Cidr) -> Self {
        Self {
            iter: Ipv4CidrU8ArrayIterator::new(cidr)
        }
    }
}

impl Ipv4CidrIterator {
    #[inline]
    pub fn nth_u64(&mut self, n: u64) -> Option<u32> {
        self.iter.nth_u64(n).map(u32::from_be_bytes)
    }
}

impl Iterator for Ipv4CidrIterator {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(u32::from_be_bytes)
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.iter.last().map(u32::from_be_bytes)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(u32::from_be_bytes)
    }
}

impl DoubleEndedIterator for Ipv4CidrIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(u32::from_be_bytes)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(u32::from_be_bytes)
    }
}

// TODO: Ipv4CidrIpv4AddrIterator

/// To iterate IPv4 CIDRs.
#[derive(Debug)]
pub struct Ipv4CidrIpv4AddrIterator {
    iter: Ipv4CidrU8ArrayIterator,
}

impl Ipv4CidrIpv4AddrIterator {
    #[inline]
    pub fn new(cidr: &Ipv4Cidr) -> Self {
        Self {
            iter: Ipv4CidrU8ArrayIterator::new(cidr)
        }
    }
}

impl Ipv4CidrIpv4AddrIterator {
    #[inline]
    pub fn nth_u64(&mut self, n: u64) -> Option<Ipv4Addr> {
        self.iter.nth_u64(n).map(|a| a.into())
    }
}

impl Iterator for Ipv4CidrIpv4AddrIterator {
    type Item = Ipv4Addr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|a| a.into())
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.iter.last().map(|a| a.into())
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|a| a.into())
    }
}

impl DoubleEndedIterator for Ipv4CidrIpv4AddrIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|a| a.into())
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|a| a.into())
    }
}
