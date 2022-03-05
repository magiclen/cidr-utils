CIDR Utils
====================

[![CI](https://github.com/magiclen/cidr-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/cidr-utils/actions/workflows/ci.yml)

This crate provides data structures and functions to deal with IPv4 CIDRs and IPv6 CIDRs.

## Examples

The format of CIDRs.

```rust
use cidr_utils::cidr::IpCidr;

assert_eq!(true, IpCidr::is_ip_cidr("192.168.1.0/24"));
assert_eq!(true, IpCidr::is_ip_cidr("192.168.1.1/24"));
assert_eq!(true, IpCidr::is_ip_cidr("192.168.1.0/0"));
assert_eq!(false, IpCidr::is_ip_cidr("192.168.1.0/33"));

assert_eq!(true, IpCidr::is_ip_cidr("192.168.1.0/255.255.255.0"));
assert_eq!(true, IpCidr::is_ip_cidr("192.168.1.0/255.255.254.0"));
assert_eq!(false, IpCidr::is_ip_cidr("192.168.1.0/255.255.255.1"));

assert_eq!(true, IpCidr::is_ip_cidr("192.168.1.0")); // 192.168.1.0/32
assert_eq!(true, IpCidr::is_ip_cidr("192.168.1"));   // 192.168.1/24
assert_eq!(true, IpCidr::is_ip_cidr("192.168"));     // 192.168/16
assert_eq!(true, IpCidr::is_ip_cidr("192"));         // 192/8

assert_eq!(true, IpCidr::is_ip_cidr("192/8"));
assert_eq!(true, IpCidr::is_ip_cidr("192/255.0.0.0"));
assert_eq!(false, IpCidr::is_ip_cidr("192/16"));
assert_eq!(false, IpCidr::is_ip_cidr("192/255.255.0.0"));

assert_eq!(true, IpCidr::is_ip_cidr("2001:4f8:3:ba::/64"));
assert_eq!(true, IpCidr::is_ip_cidr("2001:4f8:3:ba:2e0:81ff:fe22:d1f1/128"));
assert_eq!(true, IpCidr::is_ip_cidr("::ffff:1.2.3.0/120"));
assert_eq!(true, IpCidr::is_ip_cidr("::ffff:1.2.3.1/120"));
assert_eq!(false, IpCidr::is_ip_cidr("::ffff:1.2.3.0/129"));

assert_eq!(true, IpCidr::is_ip_cidr("2001:4f8:3:ba:2e0:81ff:fe22:d1f1")); // 2001:4f8:3:ba:2e0:81ff:fe22:d1f1/128
assert_eq!(true, IpCidr::is_ip_cidr("2001:4f8:3:ba::"));                  // 2001:4f8:3:ba::/128
```

Determine whether an IP is in a CIDR.

```rust
use std::net::IpAddr;
use std::str::FromStr;

use cidr_utils::cidr::IpCidr;

let cidr = IpCidr::from_str("192.168.51.0/24").unwrap();

assert_eq!(true, cidr.contains(IpAddr::from_str("192.168.51.103").unwrap()));
assert_eq!(false, cidr.contains(IpAddr::from_str("192.168.50.103").unwrap()));
```

```rust
use std::net::Ipv4Addr;

use cidr_utils::cidr::Ipv4Cidr;

let cidr = Ipv4Cidr::from_str("192.168.51.0/24").unwrap();

assert_eq!(true, cidr.contains([192, 168, 51, 103]));
assert_eq!(true, cidr.contains(Ipv4Addr::new(192, 168, 51, 103)));
assert_eq!(false, cidr.contains([192, 168, 50, 103]));

assert_eq!(256, cidr.size());
```

Combine subnetworks to supernetworks.

```rust
use cidr_utils::cidr::Ipv4Cidr;
use cidr_utils::utils::Ipv4CidrCombiner;

let mut combiner = Ipv4CidrCombiner::new();

combiner.push(Ipv4Cidr::from_str("192.168.51.100").unwrap());

assert_eq!(1, combiner.len());
assert_eq!("192.168.51.100/32".to_string(), combiner[0].to_string());

combiner.push(Ipv4Cidr::from_str("192.168.51.101").unwrap());

assert_eq!(1, combiner.len());
assert_eq!("192.168.51.100/31".to_string(), combiner[0].to_string());

combiner.push(Ipv4Cidr::from_str("192.168.51.102").unwrap());

assert_eq!(2, combiner.len());
assert_eq!("192.168.51.100/31".to_string(), combiner[0].to_string());
assert_eq!("192.168.51.102/32".to_string(), combiner[1].to_string());

combiner.push(Ipv4Cidr::from_str("192.168.51.103").unwrap());

assert_eq!(1, combiner.len());
assert_eq!("192.168.51.100/30".to_string(), combiner[0].to_string());

assert_eq!(true, combiner.contains([192, 168, 51, 102]));
assert_eq!(false, combiner.contains([192, 168, 51, 105]));

assert_eq!(4, combiner.size());
```

Separate a network into subnetworks.

```rust
use cidr_utils::cidr::Ipv4Cidr;
use cidr_utils::utils::Ipv4CidrSeparator;

let cidr = Ipv4Cidr::from_str("192.168.56.0/24").unwrap();

let result = Ipv4CidrSeparator::divide_by(&cidr, 4).unwrap();

assert_eq!(4, result.len());
assert_eq!(64, result[0].size());
assert_eq!(64, result[1].size());
assert_eq!(64, result[2].size());
assert_eq!(64, result[3].size());

assert_eq!("[192.168.56.0/26]".to_string(), result[0].to_string());
assert_eq!("[192.168.56.64/26]".to_string(), result[1].to_string());
assert_eq!("[192.168.56.128/26]".to_string(), result[2].to_string());
assert_eq!("[192.168.56.192/26]".to_string(), result[3].to_string());

let result = Ipv4CidrSeparator::divide_by(&cidr, 5).unwrap();

assert_eq!(5, result.len());
assert_eq!(51, result[0].size());
assert_eq!(51, result[1].size());
assert_eq!(51, result[2].size());
assert_eq!(51, result[3].size());
assert_eq!(52, result[4].size());

assert_eq!("[192.168.56.0/27, 192.168.56.32/28, 192.168.56.48/31, 192.168.56.50/32]".to_string(), result[0].to_string());
assert_eq!("[192.168.56.51/32, 192.168.56.52/30, 192.168.56.56/29, 192.168.56.64/27, 192.168.56.96/30, 192.168.56.100/31]".to_string(), result[1].to_string());
assert_eq!("[192.168.56.102/31, 192.168.56.104/29, 192.168.56.112/28, 192.168.56.128/28, 192.168.56.144/29, 192.168.56.152/32]".to_string(), result[2].to_string());
assert_eq!("[192.168.56.153/32, 192.168.56.154/31, 192.168.56.156/30, 192.168.56.160/27, 192.168.56.192/29, 192.168.56.200/30]".to_string(), result[3].to_string());
assert_eq!("[192.168.56.204/30, 192.168.56.208/28, 192.168.56.224/27]".to_string(), result[4].to_string());

let result = Ipv4CidrSeparator::sub_networks(&cidr, 26).unwrap();

assert_eq!(4, result.len());
assert_eq!(64, result[0].size());
assert_eq!(64, result[1].size());
assert_eq!(64, result[2].size());
assert_eq!(64, result[3].size());

assert_eq!("192.168.56.0/26".to_string(), result[0].to_string());
assert_eq!("192.168.56.64/26".to_string(), result[1].to_string());
assert_eq!("192.168.56.128/26".to_string(), result[2].to_string());
assert_eq!("192.168.56.192/26".to_string(), result[3].to_string());
```

## Serde Support

Enable the `serde` feature to support the serde framework.

```toml
[dependencies.cidr-utils]
version = "*"
features = ["serde"]
```

## Crates.io

https://crates.io/crates/cidr-utils

## Documentation

https://docs.rs/cidr-utils

## License

[MIT](LICENSE)