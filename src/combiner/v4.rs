use core::ops::Deref;
use std::net::Ipv4Addr;

use cidr::Ipv4Cidr;

use crate::Ipv4CidrSize;

/// To combine multiple IPv4 CIDRs to supernetworks.
#[derive(Debug, Clone)]
pub struct Ipv4CidrCombiner(Vec<Ipv4Cidr>);

impl Default for Ipv4CidrCombiner {
    #[inline]
    fn default() -> Self {
        Ipv4CidrCombiner::new()
    }
}

impl Deref for Ipv4CidrCombiner {
    type Target = Vec<Ipv4Cidr>;

    #[inline]
    fn deref(&self) -> &Vec<Ipv4Cidr> {
        &self.0
    }
}

impl Ipv4CidrCombiner {
    /// Create a new `Ipv4CidrCombiner` instance.
    #[inline]
    pub const fn new() -> Ipv4CidrCombiner {
        Ipv4CidrCombiner(Vec::new())
    }

    /// Create a new `Ipv4CidrCombiner` instance with a specific capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Ipv4CidrCombiner {
        Ipv4CidrCombiner(Vec::with_capacity(capacity))
    }

    /// Create a new `Ipv4CidrCombiner` instance with an existing array.
    ///
    /// # Safety
    ///
    /// You must ensure that the input array is ordered.
    #[inline]
    pub const unsafe fn from_ipv4_cidr_vec_unchecked(cidr_vec: Vec<Ipv4Cidr>) -> Ipv4CidrCombiner {
        Ipv4CidrCombiner(cidr_vec)
    }

    #[inline]
    pub fn into_ipv4_cidr_vec(self) -> Vec<Ipv4Cidr> {
        self.0
    }
}

impl Ipv4CidrCombiner {
    /// Push a CIDR into this combiner.
    pub fn push(&mut self, mut cidr: Ipv4Cidr) {
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
                                let next_prefix: u32 = next_cidr.first_address().into();
                                let prefix: u32 = cidr.first_address().into();

                                let d = next_prefix ^ prefix;

                                if d == 1 << (32 - bits) as u32 {
                                    cidr = Ipv4Cidr::new(prefix.into(), bits - 1).unwrap();

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
                                let previous_prefix: u32 = previous_cidr.first_address().into();
                                let prefix: u32 = cidr.first_address().into();

                                let d = prefix ^ previous_prefix;

                                if d == 1 << (32 - bits) as u32 {
                                    self.0.remove(index_dec);

                                    index = index_dec;

                                    cidr = Ipv4Cidr::new(previous_prefix.into(), previous_bits - 1)
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

    /// Check an IPv4 whether it is in these CIDRs.
    #[inline]
    pub fn contains(&self, ipv4: &Ipv4Addr) -> bool {
        for cidr in self.0.iter() {
            if cidr.contains(ipv4) {
                return true;
            }
        }

        false
    }

    /// Get the total size of CIDRs.
    #[inline]
    pub fn size(&self) -> u64 {
        let mut sum = 0;

        for cidr in self.0.iter() {
            sum += cidr.size();
        }

        sum
    }
}
