extern crate cidr_utils;

use cidr_utils::{cidr::Ipv4Cidr, utils::Ipv4CidrSeparator};

#[test]
fn divide_by() {
    let cidr = Ipv4Cidr::from_str("192.168.56.0/24").unwrap();

    let result = Ipv4CidrSeparator::divide_by(&cidr, 4).unwrap();

    assert_eq!(4, result.len());
    assert_eq!(64, result[0].size());
    assert_eq!(64, result[1].size());
    assert_eq!(64, result[2].size());
    assert_eq!(64, result[3].size());

    let cidr = Ipv4Cidr::from_str("192.168.56.0/24").unwrap();

    let result = Ipv4CidrSeparator::divide_by(&cidr, 5).unwrap();

    assert_eq!(5, result.len());
    assert_eq!(51, result[0].size());
    assert_eq!(51, result[1].size());
    assert_eq!(51, result[2].size());
    assert_eq!(51, result[3].size());
    assert_eq!(52, result[4].size());

    let cidr = Ipv4Cidr::from_str("0.0.0.0/0").unwrap();

    let result = Ipv4CidrSeparator::divide_by(&cidr, 1).unwrap();

    assert_eq!(1, result.len());
    assert_eq!(2u64.pow(32), result[0].size());

    let result = Ipv4CidrSeparator::divide_by(&cidr, 2).unwrap();

    assert_eq!(2, result.len());
    assert_eq!(2u64.pow(31), result[0].size());
    assert_eq!(2u64.pow(31), result[1].size());
}

#[test]
fn sub_networks() {
    let cidr = Ipv4Cidr::from_str("192.168.56.0/24").unwrap();

    let result = Ipv4CidrSeparator::sub_networks(&cidr, 26).unwrap();

    assert_eq!(4, result.len());
    assert_eq!(64, result[0].size());
    assert_eq!(64, result[1].size());
    assert_eq!(64, result[2].size());
    assert_eq!(64, result[3].size());
}
