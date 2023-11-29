use core::fmt::{self, Display, Formatter};
use std::fmt::Write;

use cidr::{Ipv4Cidr, Ipv6Cidr};

use super::{Ipv4CidrCombiner, Ipv6CidrCombiner};

pub(crate) struct DisplayIpv4Cidr<'a>(&'a Ipv4Cidr);

impl<'a> Display for DisplayIpv4Cidr<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ip = self.0.first_address();
        let bits = self.0.network_length();

        f.write_fmt(format_args!("{ip}/{bits}"))
    }
}

impl Display for Ipv4CidrCombiner {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;

        let length = self.len();

        if length > 0 {
            let length_dec = length - 1;

            for cidr in self.iter().take(length_dec) {
                f.write_fmt(format_args!("{}, ", DisplayIpv4Cidr(cidr)))?
            }

            f.write_fmt(format_args!("{}", DisplayIpv4Cidr(&self[length_dec])))?;
        }

        f.write_char(']')
    }
}

pub(crate) struct DisplayIpv6Cidr<'a>(&'a Ipv6Cidr);

impl<'a> Display for DisplayIpv6Cidr<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ip = self.0.first_address();
        let bits = self.0.network_length();

        f.write_fmt(format_args!("{ip}/{bits}"))
    }
}

impl Display for Ipv6CidrCombiner {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;

        let length = self.len();

        if length > 0 {
            let length_dec = length - 1;

            for cidr in self.iter().take(length_dec) {
                f.write_fmt(format_args!("{}, ", DisplayIpv6Cidr(cidr)))?
            }

            f.write_fmt(format_args!("{}", DisplayIpv6Cidr(&self[length_dec])))?;
        }

        f.write_char(']')
    }
}
