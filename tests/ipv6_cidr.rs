extern crate cidr_utils;

use std::cmp::Ordering;
use std::net::Ipv6Addr;
use std::str::FromStr;

use cidr_utils::cidr::Ipv6Cidr;
use cidr_utils::num_bigint::BigUint;

#[test]
fn from_prefix_and_mask() {
    let cidr_1 = Ipv6Cidr::from_prefix_and_mask(
        [0, 0, 0, 0, 0, 65535, 65535, 0],
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 0),
    )
    .unwrap();
    let cidr_2 = Ipv6Cidr::from_prefix_and_mask(
        [0, 0, 0, 0, 0, 65535, 65500, 0],
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 32768),
    )
    .unwrap();

    assert_eq!(112, cidr_1.get_bits());
    assert_eq!(
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 0),
        cidr_1.get_mask_as_ipv6_addr()
    );
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 0), cidr_1.get_prefix_as_ipv6_addr());

    assert_eq!(113, cidr_2.get_bits());
    assert_eq!(
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 32768),
        cidr_2.get_mask_as_ipv6_addr()
    );
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65500, 0), cidr_2.get_prefix_as_ipv6_addr());
}

#[test]
fn from_prefix_and_bits() {
    let cidr_1 = Ipv6Cidr::from_prefix_and_bits([0, 0, 0, 0, 0, 65535, 65535, 0], 112).unwrap();
    let cidr_2 = Ipv6Cidr::from_prefix_and_bits([0, 0, 0, 0, 0, 65535, 65500, 0], 113).unwrap();

    assert_eq!(112, cidr_1.get_bits());
    assert_eq!(
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 0),
        cidr_1.get_mask_as_ipv6_addr()
    );
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 0), cidr_1.get_prefix_as_ipv6_addr());

    assert_eq!(113, cidr_2.get_bits());
    assert_eq!(
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 32768),
        cidr_2.get_mask_as_ipv6_addr()
    );
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65500, 0), cidr_2.get_prefix_as_ipv6_addr());
}

#[test]
fn from_str() {
    let cidr_1 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();
    let cidr_2 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFDC:0/113").unwrap();

    assert_eq!(112, cidr_1.get_bits());
    assert_eq!(
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 0),
        cidr_1.get_mask_as_ipv6_addr()
    );
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 0), cidr_1.get_prefix_as_ipv6_addr());

    assert_eq!(113, cidr_2.get_bits());
    assert_eq!(
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 32768),
        cidr_2.get_mask_as_ipv6_addr()
    );
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65500, 0), cidr_2.get_prefix_as_ipv6_addr());

    let cidr_3 = Ipv6Cidr::from_str("::ffff:0.128.0.128").unwrap();

    assert_eq!(128, cidr_3.get_bits());
    assert_eq!(
        Ipv6Addr::new(65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535),
        cidr_3.get_mask_as_ipv6_addr()
    );
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 128, 128), cidr_3.get_prefix_as_ipv6_addr());
}

#[test]
fn last() {
    let cidr_1 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();
    let cidr_2 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFDC:0/113").unwrap();

    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 65535), cidr_1.last_as_ipv6_addr());
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65500, 32767), cidr_2.last_as_ipv6_addr());
}

#[test]
fn size() {
    let cidr_1 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();
    let cidr_2 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/0").unwrap();

    assert_eq!(BigUint::from(65536u128), cidr_1.size());
    assert_eq!(
        BigUint::from_str("340282366920938463463374607431768211456").unwrap(),
        cidr_2.size()
    );
}

#[test]
fn compare() {
    let cidr_1 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();
    let cidr_2 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFDC:0/113").unwrap();

    assert_eq!(Ordering::Greater, cidr_1.partial_cmp(&cidr_2).unwrap());
}

#[test]
fn contains() {
    let cidr_1 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();
    let cidr_2 = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFDC:0/113").unwrap();

    assert_eq!(false, cidr_1.contains([0, 0, 0, 0, 0, 65535, 65534, 65535]));
    assert_eq!(true, cidr_1.contains([0, 0, 0, 0, 0, 65535, 65535, 1]));
    assert_eq!(false, cidr_2.contains([0, 0, 0, 0, 0, 65535, 65500, 32768]));
    assert_eq!(true, cidr_2.contains([0, 0, 0, 0, 0, 65535, 65500, 32767]));
}

#[test]
fn iter() {
    let cidr = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();

    let mut iter = cidr.iter();

    assert_eq!(u128::from(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 0)), iter.next().unwrap());
    assert_eq!(u128::from(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 1)), iter.next().unwrap());
    assert_eq!(u128::from(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 2)), iter.next().unwrap());
    assert_eq!(u128::from(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 65535)), iter.last().unwrap());
}

#[test]
fn iter_as_ipv6_addr() {
    let cidr = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();

    let mut iter = cidr.iter_as_ipv6_addr();

    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 0), iter.next().unwrap());
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 1), iter.next().unwrap());
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 2), iter.next().unwrap());
    assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 65535, 65535), iter.last().unwrap());
}
