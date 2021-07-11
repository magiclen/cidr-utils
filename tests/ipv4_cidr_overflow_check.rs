extern crate cidr_utils;
use std::net::Ipv4Addr;
use cidr_utils::{cidr::IpCidr, utils::IpCidrSeparator};

#[test]
fn cidr_static_overflowed() {
    let x = IpCidr::from_str("0.0.0.5/24").unwrap();
    assert_eq!(x.last_as_ip_addr(), Ipv4Addr::new(0, 0, 0, 255));
}

#[test]
fn cidr_overflowed() {
    let x = IpCidr::from_str("0.0.0.5/24").unwrap();
    let last = x.iter().last().unwrap();
    
    assert_eq!(last, Ipv4Addr::new(0, 0, 0, 255));
}