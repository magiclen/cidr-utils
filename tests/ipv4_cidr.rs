use cidr_utils::cidr::Ipv4Cidr;
use std::cmp::Ordering;
use std::net::Ipv4Addr;

#[test]
fn from_prefix_and_mask() {
    let cidr_1 =
        Ipv4Cidr::from_prefix_and_mask([192, 168, 51, 1], Ipv4Addr::new(255, 255, 255, 0)).unwrap();
    let cidr_2 =
        Ipv4Cidr::from_prefix_and_mask([192, 168, 43, 1], Ipv4Addr::new(255, 255, 255, 128))
            .unwrap();

    assert_eq!(24, cidr_1.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 255, 0), cidr_1.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(192, 168, 51, 0), cidr_1.get_prefix_as_ipv4_addr());

    assert_eq!(25, cidr_2.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 255, 128), cidr_2.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(192, 168, 43, 0), cidr_2.get_prefix_as_ipv4_addr());
}

#[test]
fn from_prefix_and_bits() {
    let cidr_1 = Ipv4Cidr::from_prefix_and_bits([192, 168, 51, 1], 24).unwrap();
    let cidr_2 = Ipv4Cidr::from_prefix_and_bits([192, 168, 43, 1], 25).unwrap();

    assert_eq!(24, cidr_1.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 255, 0), cidr_1.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(192, 168, 51, 0), cidr_1.get_prefix_as_ipv4_addr());

    assert_eq!(25, cidr_2.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 255, 128), cidr_2.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(192, 168, 43, 0), cidr_2.get_prefix_as_ipv4_addr());
}

#[test]
fn from_str() {
    let cidr_1 = Ipv4Cidr::from_str("192.168.51.1/24").unwrap();
    let cidr_2 = Ipv4Cidr::from_str("192.168.43.1/25").unwrap();

    assert_eq!(24, cidr_1.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 255, 0), cidr_1.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(192, 168, 51, 0), cidr_1.get_prefix_as_ipv4_addr());

    assert_eq!(25, cidr_2.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 255, 128), cidr_2.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(192, 168, 43, 0), cidr_2.get_prefix_as_ipv4_addr());

    let cidr_3 = Ipv4Cidr::from_str("0.0.255.0/255.255.0.0").unwrap();

    assert_eq!(16, cidr_3.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 0, 0), cidr_3.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(0, 0, 0, 0), cidr_3.get_prefix_as_ipv4_addr());

    let cidr_4 = Ipv4Cidr::from_str("0.0.255.0").unwrap();

    assert_eq!(32, cidr_4.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 255, 255), cidr_4.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(0, 0, 255, 0), cidr_4.get_prefix_as_ipv4_addr());

    let cidr_5 = Ipv4Cidr::from_str("0.0.255").unwrap();

    assert_eq!(24, cidr_5.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 255, 0), cidr_5.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(0, 0, 255, 0), cidr_5.get_prefix_as_ipv4_addr());

    let cidr_6 = Ipv4Cidr::from_str("0.0").unwrap();

    assert_eq!(16, cidr_6.get_bits());
    assert_eq!(Ipv4Addr::new(255, 255, 0, 0), cidr_6.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(0, 0, 0, 0), cidr_6.get_prefix_as_ipv4_addr());

    let cidr_7 = Ipv4Cidr::from_str("0").unwrap();

    assert_eq!(8, cidr_7.get_bits());
    assert_eq!(Ipv4Addr::new(255, 0, 0, 0), cidr_7.get_mask_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(0, 0, 0, 0), cidr_7.get_prefix_as_ipv4_addr());
}

#[test]
fn last() {
    let cidr_1 = Ipv4Cidr::from_str("192.168.51.1/16").unwrap();
    let cidr_2 = Ipv4Cidr::from_str("192.168.43.1/17").unwrap();

    assert_eq!(Ipv4Addr::new(192, 168, 255, 255), cidr_1.last_as_ipv4_addr());
    assert_eq!(Ipv4Addr::new(192, 168, 127, 255), cidr_2.last_as_ipv4_addr());
}

#[test]
fn compare() {
    let cidr_1 = Ipv4Cidr::from_str("192.168.51.1/24").unwrap();
    let cidr_2 = Ipv4Cidr::from_str("192.168.43.1/25").unwrap();

    assert_eq!(Ordering::Greater, cidr_1.partial_cmp(&cidr_2).unwrap());

    let cidr_3 = Ipv4Cidr::from_str("10.0.10.254").unwrap();
    let cidr_4 = Ipv4Cidr::from_str("127.0.0.1").unwrap();

    assert_eq!(Ordering::Less, cidr_3.partial_cmp(&cidr_4).unwrap());

    let cidr_5 = Ipv4Cidr::from_str("127.0.0.1").unwrap();
    let cidr_6 = Ipv4Cidr::from_str("127.0.0.1").unwrap();
    let cidr_7 = Ipv4Cidr::from_str("127.0.0.1/31").unwrap();

    assert_eq!(Ordering::Equal, cidr_5.partial_cmp(&cidr_6).unwrap());
    assert_eq!(Ordering::Greater, cidr_5.partial_cmp(&cidr_7).unwrap());

    let cidr_8 = Ipv4Cidr::from_str("200.1.0.0/24").unwrap();
    let cidr_9 = Ipv4Cidr::from_str("192.160.0.0/12").unwrap();

    assert_eq!(Ordering::Greater, cidr_8.partial_cmp(&cidr_9).unwrap());
    assert_eq!(Ordering::Less, cidr_9.partial_cmp(&cidr_8).unwrap());
}

#[test]
fn contains() {
    let cidr_1 = Ipv4Cidr::from_str("192.168.51.1/16").unwrap();
    let cidr_2 = Ipv4Cidr::from_str("192.168.43.1/17").unwrap();

    assert!(!cidr_1.contains([127, 0, 0, 1]));
    assert!(!cidr_1.contains([192, 167, 0, 0]));
    assert!(cidr_1.contains([192, 168, 0, 0]));
    assert!(cidr_1.contains([192, 168, 51, 0]));
    assert!(cidr_1.contains([192, 168, 255, 255]));
    assert!(!cidr_1.contains([192, 169, 0, 0]));
    assert!(cidr_2.contains([192, 168, 127, 255]));
    assert!(!cidr_2.contains([192, 168, 128, 0]));
}

#[test]
fn iter() {
    let cidr = Ipv4Cidr::from_str("192.168.51.1/16").unwrap();

    let mut iter = cidr.iter();

    assert_eq!(u32::from(Ipv4Addr::new(192, 168, 0, 0)), iter.next().unwrap());
    assert_eq!(u32::from(Ipv4Addr::new(192, 168, 0, 1)), iter.next().unwrap());
    assert_eq!(u32::from(Ipv4Addr::new(192, 168, 0, 2)), iter.next().unwrap());
    assert_eq!(u32::from(Ipv4Addr::new(192, 168, 255, 255)), iter.last().unwrap());
}

#[test]
fn iter_as_ipv4_addr() {
    let cidr = Ipv4Cidr::from_str("192.168.51.1/16").unwrap();

    let mut iter = cidr.iter_as_ipv4_addr();

    assert_eq!(Ipv4Addr::new(192, 168, 0, 0), iter.next().unwrap());
    assert_eq!(Ipv4Addr::new(192, 168, 0, 1), iter.next().unwrap());
    assert_eq!(Ipv4Addr::new(192, 168, 0, 2), iter.next().unwrap());
    assert_eq!(Ipv4Addr::new(192, 168, 255, 255), iter.last().unwrap());
}

#[test]
fn iter_rev() {
    let cidr = Ipv4Cidr::from_str("192.168.51.1/16").unwrap();

    let mut iter = cidr.iter().rev();

    assert_eq!(u32::from(Ipv4Addr::new(192, 168, 255, 255)), iter.next().unwrap());
    assert_eq!(u32::from(Ipv4Addr::new(192, 168, 255, 254)), iter.next().unwrap());
    assert_eq!(u32::from(Ipv4Addr::new(192, 168, 255, 253)), iter.next().unwrap());
    assert_eq!(u32::from(Ipv4Addr::new(192, 168, 0, 0)), iter.last().unwrap());
}

#[test]
fn iter_rev_as_ipv4_addr() {
    let cidr = Ipv4Cidr::from_str("192.168.51.1/16").unwrap();

    let mut iter = cidr.iter_as_ipv4_addr().rev();

    assert_eq!(Ipv4Addr::new(192, 168, 255, 255), iter.next().unwrap());
    assert_eq!(Ipv4Addr::new(192, 168, 255, 254), iter.next().unwrap());
    assert_eq!(Ipv4Addr::new(192, 168, 255, 253), iter.next().unwrap());
    assert_eq!(Ipv4Addr::new(192, 168, 0, 0), iter.last().unwrap());
}
