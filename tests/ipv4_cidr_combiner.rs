use cidr_utils::{cidr::Ipv4Cidr, utils::Ipv4CidrCombiner};

#[test]
fn push() {
    let mut combiner = Ipv4CidrCombiner::new();

    assert_eq!(0, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.168.1.0/24").unwrap());

    assert_eq!(1, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.168.1.0/24").unwrap());

    assert_eq!(1, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.168.1.52/32").unwrap());

    assert_eq!(1, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.168.2.0/24").unwrap());

    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.168.0.0/16").unwrap());

    assert_eq!(1, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.168.3.0/24").unwrap());

    assert_eq!(1, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.171.0.0/16").unwrap());

    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.170.0.0/16").unwrap());

    assert_eq!(2, combiner.len());
    assert_eq!("192.170.0.0/15", combiner[1].to_string());

    combiner.push(Ipv4Cidr::from_str("192.169.0.0/16").unwrap());

    assert_eq!(1, combiner.len());
    assert_eq!("192.168.0.0/14", combiner[0].to_string());

    combiner.push(Ipv4Cidr::from_str("192.167.0.0/16").unwrap());
    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.166.0.0/16").unwrap());
    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.166.0.0/16").unwrap());
    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.164.0.0/15").unwrap());
    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.172.0.0/14").unwrap());
    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("192.160.0.0/14").unwrap());
    assert_eq!(1, combiner.len());

    // 192.168.1.0/24 + 192.168.1.0/24 = 192.168.1.0/24
    //
    // 192.168.1.0/24 + 192.168.2.0/24 = 192.168.1.0/24 + 192.168.2.0/24
    //
    // 192.168.1.0/24 + 192.168.2.0/24 + 192.168.0.0/16 = 192.168.0.0/16
    //
    // 192.168.0.0/16 + 192.168.3.0/24 = 192.168.0.0/16
    //
    // 192.168.0.0/16 + 192.171.0.0/16 = 192.168.0.0/16 + 192.171.0.0/16
    //
    // 192.168.0.0/16 + 192.171.0.0/16 + 192.170.0.0/16 = 192.168.0.0/16 + 192.170.0.0/15
    //
    // 192.168.0.0/16 + 192.170.0.0/15 + 192.169.0.0/16 = 192.168.0.0/15 + 192.170.0.0/15 = 192.168.0.0/14
    //
    // 192.168.0.0/14 + 192.167.0.0/16 = 192.168.0.0/14 + 192.167.0.0/16
    //
    // 192.168.0.0/14 + 192.167.0.0/16 + 192.166.0.0/16 = 192.168.0.0/14 + 192.166.0.0/15
    //
    // 192.168.0.0/14 + 192.166.0.0/15 + 192.164.0.0/15 = 192.168.0.0/14 + 192.164.0.0/14
    //
    // 192.168.0.0/14 + 192.164.0.0/14 + 192.172.0.0/14 = 192.168.0.0/14 + 192.164.0.0/13
    //
    // 192.168.0.0/14 + 192.164.0.0/13 + 192.160.0.0/14 = 192.160.0.0/13 + 192.164.0.0/13 = 192.160.0.0/12

    combiner.push(Ipv4Cidr::from_str("200.1.0.0/24").unwrap());
    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("200.1.1.0/24").unwrap());
    assert_eq!(2, combiner.len());

    combiner.push(Ipv4Cidr::from_str("0.0.0.0/1").unwrap());
    assert_eq!(3, combiner.len());

    combiner.push(Ipv4Cidr::from_str("0.0.0.0/0").unwrap());
    assert_eq!(1, combiner.len());

    // 192.160.0.0/12 + 200.1.0.0/24 = 192.160.0.0/12 + 200.1.0.0/24
    //
    // 192.160.0.0/12 + 200.1.0.0/24 + 200.1.1.0/24 = 192.160.0.0/12 + 200.1.0.0/23
    //
    // 192.160.0.0/12 + 200.1.0.0/23 + 0.0.0.0/1 = 192.160.0.0/12 + 200.1.0.0/23 + 0.0.0.0/1
    //
    // 192.160.0.0/12 + 200.1.0.0/23 + 0.0.0.0/1 + 0.0.0.0/0 = 0.0.0.0/0
}

#[test]
fn simple_test() {
    let mut combiner = Ipv4CidrCombiner::new();

    combiner.push(Ipv4Cidr::from_str("192.168.1.100").unwrap());
    combiner.push(Ipv4Cidr::from_str("192.168.1.101").unwrap());
    combiner.push(Ipv4Cidr::from_str("192.168.1.102").unwrap());
    combiner.push(Ipv4Cidr::from_str("192.168.1.103").unwrap());

    assert_eq!(1, combiner.len());
    assert_eq!(Ipv4Cidr::from_str("192.168.1.100/30").unwrap(), combiner[0]);
}
