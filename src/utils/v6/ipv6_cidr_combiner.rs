use std::fmt::{self, Display, Formatter, Write};
use std::ops::Deref;

use crate::cidr::{Ipv6Able, Ipv6Cidr};
use crate::num_bigint::BigUint;

use num_traits::Zero;

/// To combine multiple IPv6 CIDRs to supernetworks.
#[derive(Debug)]
pub struct Ipv6CidrCombiner {
    cidr_array: Vec<Ipv6Cidr>,
}

impl Ipv6CidrCombiner {
    #[inline]
    /// Create a new `Ipv6CidrCombiner` instance.
    pub fn new() -> Ipv6CidrCombiner {
        Ipv6CidrCombiner {
            cidr_array: Vec::new(),
        }
    }

    #[inline]
    /// Create a new `Ipv6CidrCombiner` instance with a specific capacity.
    pub fn with_capacity(capacity: usize) -> Ipv6CidrCombiner {
        Ipv6CidrCombiner {
            cidr_array: Vec::with_capacity(capacity),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_ipv6_cidr_vec_unchecked(cidr_vec: Vec<Ipv6Cidr>) -> Ipv6CidrCombiner {
        Ipv6CidrCombiner {
            cidr_array: cidr_vec,
        }
    }

    #[inline]
    pub fn into_ipv6_cidr_vec(self) -> Vec<Ipv6Cidr> {
        self.cidr_array
    }
}

impl Ipv6CidrCombiner {
    /// Push a CIDR into this combiner.
    pub fn push(&mut self, mut cidr: Ipv6Cidr) {
        if let Err(mut index) = self.cidr_array.binary_search(&cidr) {
            if self.cidr_array.is_empty() {
                self.cidr_array.push(cidr);
            } else {
                let pushable = if index == 0 {
                    true
                } else {
                    let previous_cidr = self.cidr_array.get(index - 1).unwrap();

                    !previous_cidr.contains(&cidr.first())
                };

                if pushable {
                    loop {
                        if index == self.cidr_array.len() {
                            break;
                        }

                        let next = self.cidr_array.get(index).unwrap();

                        if cidr.contains(next.first()) {
                            self.cidr_array.remove(index);
                        } else {
                            break;
                        }
                    }

                    let mut merging = true;

                    while merging {
                        merging = false;

                        if index < self.cidr_array.len() {
                            let next_cidr = self.cidr_array.get(index).unwrap();

                            let next_bits = next_cidr.get_bits();
                            let bits = cidr.get_bits();

                            if bits == next_bits {
                                let next_prefix = next_cidr.get_prefix();
                                let prefix = cidr.get_prefix();

                                let d = next_prefix ^ prefix;

                                if d == 1 << (128 - bits) as u128 {
                                    cidr =
                                        Ipv6Cidr::from_prefix_and_bits(prefix, bits - 1).unwrap();

                                    self.cidr_array.remove(index);

                                    merging = true;
                                }
                            }
                        }

                        if index > 0 {
                            let index_dec = index - 1;

                            let previous_cidr = self.cidr_array.get_mut(index_dec).unwrap();

                            let previous_bits = previous_cidr.get_bits();
                            let bits = cidr.get_bits();

                            if bits == previous_bits {
                                let previous_prefix = previous_cidr.get_prefix();
                                let prefix = cidr.get_prefix();

                                let d = prefix ^ previous_prefix;

                                if d == 1 << (128 - bits) as u128 {
                                    self.cidr_array.remove(index_dec);

                                    index = index_dec;

                                    cidr = Ipv6Cidr::from_prefix_and_bits(
                                        previous_prefix,
                                        previous_bits - 1,
                                    )
                                    .unwrap();

                                    merging = true;
                                }
                            }
                        }
                    }

                    self.cidr_array.insert(index, cidr);
                }
            }
        }
    }

    #[inline]
    /// Check an IPv6 whether it is in these CIDRs.
    pub fn contains<IP: Ipv6Able>(&self, ipv6: IP) -> bool {
        for cidr in self.cidr_array.iter() {
            if cidr.contains(&ipv6) {
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn size(&self) -> BigUint {
        let mut sum = BigUint::zero();

        for cidr in self.cidr_array.iter() {
            let size = cidr.size();

            sum += size;
        }

        sum
    }
}

impl Display for Ipv6CidrCombiner {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_char('[')?;

        let length = self.cidr_array.len();

        if length > 0 {
            let length_dec = length - 1;

            for cidr in self.cidr_array.iter().take(length_dec) {
                f.write_fmt(format_args!("{}, ", cidr))?
            }

            f.write_fmt(format_args!("{}", self.cidr_array[length_dec]))?;
        }

        f.write_char(']')
    }
}

impl Deref for Ipv6CidrCombiner {
    type Target = Vec<Ipv6Cidr>;

    #[inline]
    fn deref(&self) -> &Vec<Ipv6Cidr> {
        &self.cidr_array
    }
}

impl Default for Ipv6CidrCombiner {
    #[inline]
    fn default() -> Self {
        Ipv6CidrCombiner::new()
    }
}
