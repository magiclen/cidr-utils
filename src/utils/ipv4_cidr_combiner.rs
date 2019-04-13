use crate::cidr::Ipv4Cidr;
use std::ops::Deref;
use std::fmt::{self, Formatter, Display};
use core::fmt::Write;

#[derive(Debug)]
pub struct Ipv4CidrCombiner {
    cidr_array: Vec<Ipv4Cidr>
}

impl Display for Ipv4CidrCombiner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_char('[')?;

        let length_dec = self.cidr_array.len() - 1;

        for cidr in self.cidr_array.iter().take(length_dec) {
            f.write_fmt(format_args!("{}, ", cidr))?
        }

        f.write_fmt(format_args!("{}", self.cidr_array[length_dec]))?;

        f.write_char(']')
    }
}

impl Deref for Ipv4CidrCombiner {
    type Target = Vec<Ipv4Cidr>;

    #[inline]
    fn deref(&self) -> &Vec<Ipv4Cidr> {
        &self.cidr_array
    }
}

impl Ipv4CidrCombiner {
    #[inline]
    pub fn new() -> Ipv4CidrCombiner {
        Ipv4CidrCombiner {
            cidr_array: Vec::new()
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Ipv4CidrCombiner {
        Ipv4CidrCombiner {
            cidr_array: Vec::with_capacity(capacity)
        }
    }
}

impl Ipv4CidrCombiner {
    pub fn push(&mut self, mut cidr: Ipv4Cidr) {
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

                                if d == 1 << ((bits - 1) / 8) * 8 + 7 >> ((bits - 1) % 8) {
                                    cidr = Ipv4Cidr::from_prefix_and_bits(prefix, bits - 1).unwrap();

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

                                if d == 1 << ((bits - 1) / 8) * 8 + 7 >> ((bits - 1) % 8) {
                                    self.cidr_array.remove(index_dec);

                                    index = index_dec;

                                    cidr = Ipv4Cidr::from_prefix_and_bits(previous_prefix, previous_bits - 1).unwrap();

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
}