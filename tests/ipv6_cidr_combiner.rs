#![cfg(feature = "combiner")]

use core::str::FromStr;

use cidr::Ipv6Cidr;
use cidr_utils::combiner::Ipv6CidrCombiner;

#[test]
fn simple_test() {
    let mut combiner = Ipv6CidrCombiner::new();

    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.100").unwrap());
    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.101").unwrap());
    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.102").unwrap());
    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.103").unwrap());

    assert_eq!(1, combiner.len());
    assert_eq!("::ffff:192.168.1.100/126", combiner[0].to_string());
}

#[test]
fn should_combine_same_ip() {
    let mut combiner = Ipv6CidrCombiner::new();

    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.100").unwrap());
    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.100").unwrap());
    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.100").unwrap());

    assert_eq!(1, combiner.len());
    assert_eq!("::ffff:192.168.1.100", combiner[0].to_string());
}
