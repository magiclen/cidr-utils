CIDR Utils
====================

[![CI](https://github.com/magiclen/cidr-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/cidr-utils/actions/workflows/ci.yml)

This crate provides functions for working with IPv4 CIDRs and IPv6 CIDRs.

## Examples

Combine subnetworks to supernetworks.

```rust
use std::str::FromStr;

use cidr::Ipv4Cidr;
use cidr_utils::Ipv4CidrSize;
use cidr_utils::combiner::Ipv4CidrCombiner;

let mut combiner = Ipv4CidrCombiner::new();

combiner.push(Ipv4Cidr::from_str("192.168.51.100").unwrap());

assert_eq!(1, combiner.len());
assert_eq!("192.168.51.100".to_string(), combiner[0].to_string());

combiner.push(Ipv4Cidr::from_str("192.168.51.101").unwrap());

assert_eq!(1, combiner.len());
assert_eq!("192.168.51.100/31".to_string(), combiner[0].to_string());

combiner.push(Ipv4Cidr::from_str("192.168.51.102").unwrap());

assert_eq!(2, combiner.len());
assert_eq!("192.168.51.100/31".to_string(), combiner[0].to_string());
assert_eq!("192.168.51.102".to_string(), combiner[1].to_string());

combiner.push(Ipv4Cidr::from_str("192.168.51.103").unwrap());

assert_eq!(1, combiner.len());
assert_eq!("192.168.51.100/30".to_string(), combiner[0].to_string());

assert_eq!(true, combiner.contains(&[192, 168, 51, 102].into()));
assert_eq!(false, combiner.contains(&[192, 168, 51, 105].into()));

assert_eq!(4, combiner.size());
```

Separate a network into subnetworks.

```rust
use std::str::FromStr;

use cidr::Ipv4Cidr;
use cidr_utils::Ipv4CidrSize;
use cidr_utils::separator::Ipv4CidrSeparator;

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

## Crates.io

https://crates.io/crates/cidr-utils

## Documentation

https://docs.rs/cidr-utils

## License

[MIT](LICENSE)