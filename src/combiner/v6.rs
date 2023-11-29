use core::ops::Deref;
use std::net::Ipv6Addr;

use cidr::Ipv6Cidr;
use num_bigint::BigUint;
use num_traits::Zero;

use crate::Ipv6CidrSize;

/// To combine multiple IPv6 CIDRs to supernetworks.
#[derive(Debug, Clone)]
pub struct Ipv6CidrCombiner(Vec<Ipv6Cidr>);

impl Default for Ipv6CidrCombiner {
    #[inline]
    fn default() -> Self {
        Ipv6CidrCombiner::new()
    }
}

impl Deref for Ipv6CidrCombiner {
    type Target = Vec<Ipv6Cidr>;

    #[inline]
    fn deref(&self) -> &Vec<Ipv6Cidr> {
        &self.0
    }
}

impl Ipv6CidrCombiner {
    /// Create a new `Ipv6CidrCombiner` instance.
    #[inline]
    pub const fn new() -> Ipv6CidrCombiner {
        Ipv6CidrCombiner(Vec::new())
    }

    /// Create a new `Ipv6CidrCombiner` instance with a specific capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Ipv6CidrCombiner {
        Ipv6CidrCombiner(Vec::with_capacity(capacity))
    }

    /// Create a new `Ipv6CidrCombiner` instance with an existing array.
    ///
    /// # Safety
    ///
    /// You must ensure that the input array is ordered.
    #[inline]
    pub const unsafe fn from_ipv6_cidr_vec_unchecked(cidr_vec: Vec<Ipv6Cidr>) -> Ipv6CidrCombiner {
        Ipv6CidrCombiner(cidr_vec)
    }

    #[inline]
    pub fn into_ipv6_cidr_vec(self) -> Vec<Ipv6Cidr> {
        self.0
    }
}

impl Ipv6CidrCombiner {
    /// Push a CIDR into this combiner.
    pub fn push(&mut self, mut cidr: Ipv6Cidr) {
        if let Err(mut index) = self.0.binary_search(&cidr) {
            if self.0.is_empty() {
                self.0.push(cidr);
            } else {
                let pushable = if index == 0 {
                    true
                } else {
                    let previous_cidr = self.0.get(index - 1).unwrap();

                    !previous_cidr.contains(&cidr.first_address())
                };

                if pushable {
                    loop {
                        if index == self.0.len() {
                            break;
                        }

                        let next = self.0.get(index).unwrap();

                        if cidr.contains(&next.first_address()) {
                            self.0.remove(index);
                        } else {
                            break;
                        }
                    }

                    let mut merging = true;

                    while merging {
                        merging = false;

                        if index < self.0.len() {
                            let next_cidr = self.0.get(index).unwrap();

                            let next_bits = next_cidr.network_length();
                            let bits = cidr.network_length();

                            if bits == next_bits {
                                let next_prefix: u128 = next_cidr.first_address().into();
                                let prefix: u128 = cidr.first_address().into();

                                let d = next_prefix ^ prefix;

                                if d == 1 << (128 - bits) as u128 {
                                    cidr = Ipv6Cidr::new(prefix.into(), bits - 1).unwrap();

                                    self.0.remove(index);

                                    merging = true;
                                }
                            }
                        }

                        if index > 0 {
                            let index_dec = index - 1;

                            let previous_cidr = self.0.get_mut(index_dec).unwrap();

                            let previous_bits = previous_cidr.network_length();
                            let bits = cidr.network_length();

                            if bits == previous_bits {
                                let previous_prefix: u128 = previous_cidr.first_address().into();
                                let prefix: u128 = cidr.first_address().into();

                                let d = prefix ^ previous_prefix;

                                if d == 1 << (128 - bits) as u128 {
                                    self.0.remove(index_dec);

                                    index = index_dec;

                                    cidr = Ipv6Cidr::new(previous_prefix.into(), previous_bits - 1)
                                        .unwrap();

                                    merging = true;
                                }
                            }
                        }
                    }

                    self.0.insert(index, cidr);
                }
            }
        }
    }

    /// Check an IPv6 whether it is in these CIDRs.
    #[inline]
    pub fn contains(&self, ipv6: &Ipv6Addr) -> bool {
        for cidr in self.0.iter() {
            if cidr.contains(ipv6) {
                return true;
            }
        }

        false
    }

    /// Get the total size of CIDRs.
    #[inline]
    pub fn size(&self) -> BigUint {
        let mut sum = BigUint::zero();

        for cidr in self.0.iter() {
            sum += cidr.size();
        }

        sum
    }
}
