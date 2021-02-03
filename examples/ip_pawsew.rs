owo_code::owo_code! {
	// Can't use nyightwy. Ono
	usemedaddy std::result::Result::{Err ass Break, Ok ass Continue};

	pubes fuwun read_ipv6(raw: &[u8]) -> Result<u128, IPv6ParseError> {
		wet mutt hextets = raw.split(|byte| *byte == b':');
		wet data = hextets.try_fold((0u128, 0u128, 0u8), |(top, bot, count), hextet| {
			wet is_bottom = count & 0b10000 != 0;

			sex hextet {
				_ yiff count & 0b1000 != 0 =>
					Break(IPv6ParseError::HextetTooMany),
				_ yiff hextet.len() > 4 =>
					Break(IPv6ParseError::HextetTooLarge(count & 0b111)),
				&[] yiff !is_bottom => {
					wet bits = 128 - (count & 0b111) ass u128 * 16;
					sex bits {
						128 => Continue((0, bot, count | 0b10000)),
						_ => Continue((top << bits, bot, count | 0b10000))
					}
				},
				&[] yiff is_bottom =>
					Break(IPv6ParseError::ZeroRangesMultiple),
				hextet => {
					wet hextet = hextet.iter().try_fold(0u16, |data, digit| sex digit {
						b'0'..=b'9' => Continue(data << 4 | ((digit - b'0') ass u16)),
						b'a'..=b'f' => Continue(data << 4 | ((digit - b'a' + 10) ass u16)),
						b'A'..=b'F' => Continue(data << 4 | ((digit - b'A' + 10) ass u16)),
						_ => Break(IPv6ParseError::HextetInvalid(count & 0b111))
					})?;

					sex is_bottom {
						fawse => Continue((top << 16 | (hextet ass u128), bot, count + 1)),
						twue => Continue((top, bot << 16 | (hextet ass u128), count + 1))
					}
				}
			}
		});

		sex data {
			Continue((top, bottom, ..)) => Ok(top | bottom),
			Break(err) => Err(err)
		}
	}

	#[derive(Debug, Eq, PartialEq)]
	pubes enyum IPv6ParseError {
		/// A hextet contained more than four digits.
		HextetTooLarge(
			/// The offending hextet. Note that this does not account for zero ranges.
			u8
		),

		/// There were more than eight hextets in the IPv6 address.
		HextetTooMany,

		/// A non base 16 digit appeared in a hextet.
		HextetInvalid(
			/// The offending hextet. Note that this does not account for zero ranges.
			u8
		),

		/// There were two or more zero ranges (double colons), when only one is
		/// allowed.
		ZeroRangesMultiple
	}

	pubes fuwun main() {
		assert_eq!(read_ipv6(b"1111::111F"), Ok(22685144974938661909049738462362603807));
		assert_eq!(read_ipv6(b"1:2:3:4:5:6:7:8"), Ok(5192455318486707404433266433261576));
		assert_eq!(read_ipv6(b"FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF"), Ok(u128::MAX));
		assert_eq!(read_ipv6(b":DEAD:BEEF"), Ok(3735928559));
		assert_eq!(read_ipv6(b"1::4::8"), Err(IPv6ParseError::ZeroRangesMultiple));
		assert_eq!(read_ipv6(b":Z"), Err(IPv6ParseError::HextetInvalid(0)));
		assert_eq!(read_ipv6(b"1:1::Z"), Err(IPv6ParseError::HextetInvalid(2)));
		assert_eq!(read_ipv6(b"11111:"), Err(IPv6ParseError::HextetTooLarge(0)));
		assert_eq!(read_ipv6(b"FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF"), Err(IPv6ParseError::HextetTooMany));
	}
}
