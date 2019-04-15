extern crate cidr_utils;

use cidr_utils::{cidr::IpCidr, utils::IpCidrSeparator};

#[test]
fn divide_by() {
    let cidr = IpCidr::from_str("192.168.56.0/24").unwrap();

    let result = IpCidrSeparator::divide_by(&cidr, 4).unwrap();

    assert_eq!(4, result.len());

    let cidr = IpCidr::from_str("192.168.56.0/24").unwrap();

    let result = IpCidrSeparator::divide_by(&cidr, 5).unwrap();

    assert_eq!(5, result.len());

    let cidr = IpCidr::from_str("0.0.0.0/0").unwrap();

    let result = IpCidrSeparator::divide_by(&cidr, 1).unwrap();

    assert_eq!(1, result.len());

    let result = IpCidrSeparator::divide_by(&cidr, 2).unwrap();

    assert_eq!(2, result.len());

    let cidr = IpCidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();

    let result = IpCidrSeparator::divide_by(&cidr, 4).unwrap();

    assert_eq!(4, result.len());

    let cidr = IpCidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();

    let result = IpCidrSeparator::divide_by(&cidr, 5).unwrap();

    assert_eq!(5, result.len());

    let cidr = IpCidr::from_str("::0/0").unwrap();

    let result = IpCidrSeparator::divide_by(&cidr, 1).unwrap();

    assert_eq!(1, result.len());

    let result = IpCidrSeparator::divide_by(&cidr, 2).unwrap();

    assert_eq!(2, result.len());
}

#[test]
fn sub_networks() {
    let cidr = IpCidr::from_str("192.168.56.0/24").unwrap();

    let result = IpCidrSeparator::sub_networks(&cidr, 26).unwrap();

    assert_eq!(4, result.len());

    let cidr = IpCidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();

    let result = IpCidrSeparator::sub_networks(&cidr, 114).unwrap();

    assert_eq!(4, result.len());
}