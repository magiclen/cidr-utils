use std::net::IpAddr;

use super::{IpCidr, Ipv4CidrIpv4AddrIterator, Ipv6CidrIpv6AddrIterator};

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

impl DoubleEndedIterator for IpCidrIpAddrIterator {
    #[inline]
    fn next_back(&mut self) -> Option<IpAddr> {
        match &mut self.iter {
            IpCidrIpsAddrIterator::V4(iter) => iter.next_back().map(IpAddr::V4),
            IpCidrIpsAddrIterator::V6(iter) => iter.next_back().map(IpAddr::V6),
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<IpAddr> {
        match &mut self.iter {
            IpCidrIpsAddrIterator::V4(iter) => iter.nth_back(n).map(IpAddr::V4),
            IpCidrIpsAddrIterator::V6(iter) => iter.nth_back(n).map(IpAddr::V6),
        }
    }
}

impl IpCidr {
    #[inline]
    pub fn iter_as_ip_addr(&self) -> IpCidrIpAddrIterator {
        match self {
            IpCidr::V4(cidr) => IpCidrIpAddrIterator {
                iter: IpCidrIpsAddrIterator::V4(cidr.iter_as_ipv4_addr()),
            },
            IpCidr::V6(cidr) => IpCidrIpAddrIterator {
                iter: IpCidrIpsAddrIterator::V6(cidr.iter_as_ipv6_addr()),
            },
        }
    }

    #[inline]
    pub fn iter(&self) -> IpCidrIpAddrIterator {
        self.iter_as_ip_addr()
    }
}
