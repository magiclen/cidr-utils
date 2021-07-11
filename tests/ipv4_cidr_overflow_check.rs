extern crate cidr_utils;
use std::net::{IpAddr, Ipv4Addr};
use cidr_utils::{cidr::IpCidr, utils::IpCidrSeparator};

#[test]
fn cidr_static_overflowed() {
    let x = IpCidr::from_str("0.0.0.5/24").unwrap();
    assert_eq!(x.last_as_ip_addr(), Ipv4Addr::new(0, 0, 0, 255));
}

#[test]
fn cidr_overflowed() {
    let cdir_1 = IpCidr::from_str("0.0.0.5/24").unwrap();
    let last_1 = cdir_1.iter().last().unwrap();

    assert_eq!(last_1, Ipv4Addr::new(0, 0, 0, 255));

    let cdir_2 = IpCidr::from_str("0.0.0.255/24").unwrap();
    let last_2 = cdir_2.iter().last().unwrap();

    assert_eq!(last_2, Ipv4Addr::new(0, 0, 0, 255));

    let cdir_3 = IpCidr::from_str("0.0.5.255/16").unwrap();
    let last_3 = cdir_3.iter().last().unwrap();

    assert_eq!(last_3, Ipv4Addr::new(0, 0, 255, 255));

    let cdir_4 = IpCidr::from_str("0.5.5.255/8").unwrap();
    let last_4 = cdir_4.iter().last().unwrap();
    assert_eq!(last_4, Ipv4Addr::new(0, 255, 255, 255));


    let cdir_5 = IpCidr::from_str("0.5.5.255/0").unwrap();
    let last_5 = cdir_5.iter().last().unwrap();
    assert_eq!(last_5, Ipv4Addr::new(255, 255, 255, 255));
}
