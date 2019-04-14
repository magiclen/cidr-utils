/*!
# CIDR Utils

This crate provides data structures and functions to deal with IPv4 CIDRs and IPv6 CIDRs.

## Examples

```rust
extern crate cidr_utils;

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

```rust
extern crate cidr_utils;

use std::net::IpAddr;
use std::str::FromStr;

use cidr_utils::cidr::IpCidr;

let cidr = IpCidr::from_str("192.168.51.0/24").unwrap();

assert_eq!(true, cidr.contains(IpAddr::from_str("192.168.51.103").unwrap()));
assert_eq!(false, cidr.contains(IpAddr::from_str("192.168.50.103").unwrap()));
```

```rust
extern crate cidr_utils;

use std::net::Ipv4Addr;

use cidr_utils::cidr::Ipv4Cidr;

let cidr = Ipv4Cidr::from_str("192.168.51.0/24").unwrap();

assert_eq!(true, cidr.contains([192, 168, 51, 103]));
assert_eq!(true, cidr.contains(Ipv4Addr::new(192, 168, 51, 103)));
assert_eq!(false, cidr.contains([192, 168, 50, 103]));

assert_eq!(256, cidr.size());
```

```rust
extern crate cidr_utils;

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
*/

#[macro_use]
extern crate lazy_static;
extern crate regex;

/// This module provides data structures for IPv4 CIDRs and IPv6 CIDRs.
pub mod cidr;

/// This module provides data structures and functions for specific usage.
pub mod utils;