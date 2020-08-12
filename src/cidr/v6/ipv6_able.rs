use std::net::Ipv6Addr;

use super::functions::*;

/// The type which can be taken as an IPv6 address.
/// *An `u128` value represents an IPv6 byte array (`[u8; 16]`) in big-endian (BE) order.*
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
        u128::from_be_bytes(*self)
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
        u16_array_to_u128(self.segments())
    }
}

impl<T: Ipv6Able> Ipv6Able for &T {
    #[inline]
    fn get_u128(&self) -> u128 {
        Ipv6Able::get_u128(*self)
    }
}
