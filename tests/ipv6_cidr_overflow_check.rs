extern crate cidr_utils;
use std::net::Ipv6Addr;
use cidr_utils::{cidr::IpCidr, utils::IpCidrSeparator};

#[test]
fn cidr_static_overflowed() {
    let x = IpCidr::from_str("::2/64").unwrap();
    assert_eq!(x.last_as_ip_addr(), Ipv6Addr::new(0, 0, 0, 0, u16::MAX, u16::MAX, u16::MAX, u16::MAX));
}

#[test]
fn cidr_overflowed() {
    let x = IpCidr::from_str("::2/64").unwrap();
    let last = x.iter().last().unwrap();
    
    assert_eq!(last, Ipv6Addr::new(0, 0, 0, 0, u16::MAX, u16::MAX, u16::MAX, u16::MAX));

    let x = IpCidr::from_str("::2/0").unwrap();
    let last = x.iter().last().unwrap();
    
    assert_eq!(last, Ipv6Addr::new(
        u16::MAX, u16::MAX, u16::MAX, u16::MAX,
        u16::MAX, u16::MAX, u16::MAX, u16::MAX
    ));
}