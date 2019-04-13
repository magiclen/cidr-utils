extern crate cidr_utils;

use cidr_utils::cidr::Ipv6Cidr;
use std::net::Ipv6Addr;
use std::cmp::Ordering;

#[test]
fn from_prefix_and_mask() {
    let cidr_1 = Ipv6Cidr::from_prefix_and_mask([0, 0, 0, 0, 0, 65535, 65535, 0], Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 0)).unwrap();
    let cidr_2 = Ipv6Cidr::from_prefix_and_mask([0, 0, 0, 0, 0, 65535, 65500, 0], Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 32768)).unwrap();

    assert_eq!(112, cidr_1.get_bits());
    assert_eq!(Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 0), cidr_1.get_mask_as_ipv6_addr());
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 0), cidr_1.get_prefix_as_ipv6_addr());

    assert_eq!(113, cidr_2.get_bits());
    assert_eq!(Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 32768), cidr_2.get_mask_as_ipv6_addr());
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65500, 0), cidr_2.get_prefix_as_ipv6_addr());
}

#[test]
fn from_prefix_and_bits() {
    let cidr_1 = Ipv6Cidr::from_prefix_and_bits([0, 0, 0, 0, 0, 65535, 65535, 0], 112).unwrap();
    let cidr_2 = Ipv6Cidr::from_prefix_and_bits([0, 0, 0, 0, 0, 65535, 65500, 0], 113).unwrap();

    assert_eq!(112, cidr_1.get_bits());
    assert_eq!(Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 0), cidr_1.get_mask_as_ipv6_addr());
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 0), cidr_1.get_prefix_as_ipv6_addr());

    assert_eq!(113, cidr_2.get_bits());
    assert_eq!(Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 32768), cidr_2.get_mask_as_ipv6_addr());
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65500, 0), cidr_2.get_prefix_as_ipv6_addr());
}

#[test]
fn from_str() {}