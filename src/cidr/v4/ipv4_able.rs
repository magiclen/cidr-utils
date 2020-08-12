use std::net::Ipv4Addr;

/// The type which can be taken as an IPv4 address.
/// *An `u32` value represents an IPv4 byte array (`[u8; 4]`) in big-endian (BE) order.*
pub trait Ipv4Able {
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
        u32::from_be_bytes(*self)
    }
}

impl Ipv4Able for Ipv4Addr {
    #[inline]
    fn get_u32(&self) -> u32 {
        u32::from_be_bytes(self.octets())
    }
}

impl<T: Ipv4Able> Ipv4Able for &T {
    #[inline]
    fn get_u32(&self) -> u32 {
        Ipv4Able::get_u32(*self)
    }
}
