extern crate cidr_utils;
use std::net::{IpAddr, Ipv4Addr};
use cidr_utils::{cidr::IpCidr, utils::IpCidrSeparator};

#[test]
fn cidr_starts_at_netmask() {
    let x = IpCidr::from_str("0.0.0.5/24").unwrap();
    assert_eq!(x.first_as_ip_addr(), IpAddr::from(Ipv4Addr::new(0, 0, 0, 5)));
}

#[test]
fn first_in_cidr() {
    let cdir = IpCidr::from_str("0.0.0.5/24").unwrap();
    assert_eq!(cdir.first_ip_in_cidr(), IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)));
}