// Copyright (C) 2024 Ruslan Iusupov <https://github.com/rus0000>
// SPDX-License-Identifier: MIT

/// # Parse int from string containing decimal or hex number.
/// - Beginning "0x" prefix must have lowercase "x".
/// - Explicit sign in hex number is not supported. Bad example "-0xAB" or "+0xAB".
/// - Explicit sign in decimal number is supported. Example "-12345" or "+12345".
pub fn maybe_hex<T>(arg_string: &str) -> Result<T, <T as num_traits::Num>::FromStrRadixErr>
    where T: num_traits::Num + std::str::FromStr
{
    let parsed_num: T = match arg_string.parse() {
        Err(_) => {
            let without_prefix = arg_string.trim_start_matches("0x");
            <T>::from_str_radix(without_prefix, 16)?
        }
        Ok(value) => value,
    };

    Ok(parsed_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>;

    #[test]
    fn test_u8() -> Result<()> {
        let result: u8 = maybe_hex("0xAB")?;
        assert_eq!(result, 0xAB);

        Ok(())
    }
    #[test]
    fn test_u16() -> Result<()> {
        let result: u16 = maybe_hex("0x12AB")?;
        assert_eq!(result, 0x12AB);

        Ok(())
    }
    #[test]
    fn test_u32() -> Result<()> {
        let result: u32 = maybe_hex("0x1234ABCD")?;
        assert_eq!(result, 0x1234ABCD);

        Ok(())
    }
    #[test]
    fn test_i32() -> Result<()> {
        let result: i32 = maybe_hex("0x1234ABCD")?;
        assert_eq!(result, 0x1234ABCD);

        Ok(())
    }
    #[test]
    fn test_decimal() -> Result<()> {
        let result: i32 = maybe_hex("12345678")?;
        assert_eq!(result, 12345678);

        Ok(())
    }
    #[test]
    fn test_negative_decimal() -> Result<()> {
        let result: i32 = maybe_hex("-12345678")?;
        assert_eq!(result, -12345678);

        Ok(())
    }
    #[test]
    fn test_positive_decimal() -> Result<()> {
        let result: i32 = maybe_hex("+12345678")?;
        assert_eq!(result, 12345678);

        Ok(())
    }
    #[test]
    fn test_u64() -> Result<()> {
        let result: u64 = maybe_hex("0x12345678ABCDEEFF")?;
        assert_eq!(result, 0x12345678ABCDEEFF);

        Ok(())
    }
    #[test]
    fn test_u128() -> Result<()> {
        let result: u128 = maybe_hex("0x12345678ABCDEEFF12345678ABCDEEFF")?;
        assert_eq!(result, 0x12345678ABCDEEFF12345678ABCDEEFF);

        Ok(())
    }
    #[test]
    fn test_lower_case() -> Result<()> {
        let result: u32 = maybe_hex("0x10ab")?;
        assert_eq!(result, 0x10AB);

        Ok(())
    }
    #[test]
    fn test_wrong_prefix() -> Result<()> {
        let result = maybe_hex::<u32>("0X10AB");
        assert!(result.is_err());

        Ok(())
    }
    #[test]
    fn test_overflow() -> Result<()> {
        let result = maybe_hex::<u16>("0x10ABCDEF");
        assert!(result.is_err());

        Ok(())
    }
    #[test]
    fn test_explicit_minus() -> Result<()> {
        let result = maybe_hex::<i16>("-0xAB");
        assert!(result.is_err());

        Ok(())
    }
    #[test]
    fn test_explicit_plus() -> Result<()> {
        let result = maybe_hex::<i16>("+0xAB");
        assert!(result.is_err());

        Ok(())
    }
    #[test]
    fn test_trailing_zero() -> Result<()> {
        let result: u32 = maybe_hex("0x12000000")?;
        assert_eq!(result, 0x12000000);

        Ok(())
    }
    #[test]
    fn test_leading_zero() -> Result<()> {
        let result: u32 = maybe_hex("0x00000012")?;
        assert_eq!(result, 0x12);

        Ok(())
    }
}
