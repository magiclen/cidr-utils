use cidr_utils::{
    cidr::{IpCidr, Ipv4Cidr, Ipv6Cidr},
    utils::IpCidrCombiner,
};

#[test]
fn simple_test() {
    let mut combiner = IpCidrCombiner::new();

    combiner.push(IpCidr::from_str("192.168.1.100").unwrap());
    combiner.push(IpCidr::from_str("192.168.1.101").unwrap());
    combiner.push(IpCidr::from_str("192.168.1.102").unwrap());
    combiner.push(IpCidr::from_str("192.168.1.103").unwrap());

    combiner.push(IpCidr::from_str("::ffff:192.168.1.100").unwrap());
    combiner.push(IpCidr::from_str("::ffff:192.168.1.101").unwrap());
    combiner.push(IpCidr::from_str("::ffff:192.168.1.102").unwrap());
    combiner.push(IpCidr::from_str("::ffff:192.168.1.103").unwrap());

    assert_eq!(1, combiner.get_ipv4_cidrs().len());
    assert_eq!(1, combiner.get_ipv6_cidrs().len());
    assert_eq!(Ipv4Cidr::from_str("192.168.1.100/30").unwrap(), combiner.get_ipv4_cidrs()[0]);
    assert_eq!(
        Ipv6Cidr::from_str("::ffff:192.168.1.100/126").unwrap(),
        combiner.get_ipv6_cidrs()[0]
    );
}
