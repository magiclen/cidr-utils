extern crate cidr_utils;

use std::str::FromStr;

use cidr_utils::num_bigint::BigUint;
use cidr_utils::{cidr::Ipv6Cidr, utils::Ipv6CidrSeparator};

#[test]
fn divide_by() {
    let cidr = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();

    let result = Ipv6CidrSeparator::divide_by(&cidr, 4).unwrap();

    assert_eq!(4, result.len());
    assert_eq!(BigUint::from(16384u128), result[0].size());
    assert_eq!(BigUint::from(16384u128), result[1].size());
    assert_eq!(BigUint::from(16384u128), result[2].size());
    assert_eq!(BigUint::from(16384u128), result[3].size());

    let cidr = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();

    let result = Ipv6CidrSeparator::divide_by(&cidr, 5).unwrap();

    assert_eq!(5, result.len());
    assert_eq!(BigUint::from(13107u128), result[0].size());
    assert_eq!(BigUint::from(13107u128), result[1].size());
    assert_eq!(BigUint::from(13107u128), result[2].size());
    assert_eq!(BigUint::from(13107u128), result[3].size());
    assert_eq!(BigUint::from(13108u128), result[4].size());

    let cidr = Ipv6Cidr::from_str("::0/0").unwrap();

    let result = Ipv6CidrSeparator::divide_by(&cidr, 1).unwrap();

    assert_eq!(1, result.len());
    assert_eq!(
        BigUint::from_str("340282366920938463463374607431768211456").unwrap(),
        result[0].size()
    );

    let result = Ipv6CidrSeparator::divide_by(&cidr, 2).unwrap();

    assert_eq!(2, result.len());
    assert_eq!(BigUint::from(2u128.pow(127)), result[0].size());
    assert_eq!(BigUint::from(2u128.pow(127)), result[1].size());
}

#[test]
fn sub_networks() {
    let cidr = Ipv6Cidr::from_str("0:0:0:0:0:FFFF:FFFF:0/112").unwrap();

    let result = Ipv6CidrSeparator::sub_networks(&cidr, 114).unwrap();

    assert_eq!(4, result.len());
    assert_eq!(BigUint::from(16384u128), result[0].size());
    assert_eq!(BigUint::from(16384u128), result[1].size());
    assert_eq!(BigUint::from(16384u128), result[2].size());
    assert_eq!(BigUint::from(16384u128), result[3].size());
}
