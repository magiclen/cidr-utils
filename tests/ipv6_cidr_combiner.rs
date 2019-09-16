extern crate cidr_utils;

use cidr_utils::{cidr::Ipv6Cidr, utils::Ipv6CidrCombiner};

#[test]
fn simple_test() {
    let mut combiner = Ipv6CidrCombiner::new();

    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.100").unwrap());
    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.101").unwrap());
    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.102").unwrap());
    combiner.push(Ipv6Cidr::from_str("::ffff:192.168.1.103").unwrap());

    assert_eq!(1, combiner.len());
    assert_eq!(Ipv6Cidr::from_str("::ffff:192.168.1.100/126").unwrap(), combiner[0]);
}
