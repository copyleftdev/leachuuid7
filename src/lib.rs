//! # leachuuid7
//!
//! A UUIDv7 generator conforming to the proposed UUID-7 specification.
//!
//! The UUID layout is as follows (total 128 bits):
//!
//! - **60 bits:** Unix timestamp in milliseconds (since the Unix epoch)
//! - **4 bits:** Version (always 7)
//! - **2 bits:** Variant (always binary `10`)
//! - **62 bits:** Random
//!
//! The canonical string representation is in the format:  
//! `8-4-4-4-12` hexadecimal digits.
//!
//! ## Example
//!
//! ```rust
//! use leachuuid7::Uuid7;
//!
//! let uuid = Uuid7::new();
//! println!("Generated UUIDv7: {}", uuid);
//!
//! // Parsing from a string validates the version and variant fields.
//! let parsed: Uuid7 = "0184e1a0-7e2a-7d40-8f3b-5c1a2b3c4d5e".parse()
//!     .expect("Failed to parse UUIDv7");
//! ```

use rand::Rng;
use std::fmt;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

/// A UUIDv7 value.
///
/// Internally stored as a `u128`, the bits are allocated as:
/// - **60 bits:** Unix timestamp (milliseconds since the Unix epoch)
/// - **4 bits:** Version (always 7)
/// - **2 bits:** Variant (always binary `10`)
/// - **62 bits:** Random
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Uuid7(u128);

impl Uuid7 {
    /// Generates a new UUIDv7 using the default random number generator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leachuuid7::Uuid7;
    /// let uuid = Uuid7::new();
    /// println!("{}", uuid);
    /// ```
    pub fn new() -> Self {
        Self::new_with_rng(&mut rand::thread_rng())
    }

    /// Generates a new UUIDv7 using a custom random number generator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rand::rngs::OsRng;
    /// use leachuuid7::Uuid7;
    ///
    /// let mut rng = OsRng;
    /// let uuid = Uuid7::new_with_rng(&mut rng);
    /// println!("{}", uuid);
    /// ```
    pub fn new_with_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        // Get the current time since the Unix epoch.
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        // Convert to milliseconds and ensure it fits into 60 bits.
        let ms = now.as_millis() as u64;
        let timestamp = ms & ((1u64 << 60) - 1);

        // Generate 62 random bits.
        let random62 = rng.gen::<u64>() & ((1u64 << 62) - 1);

        // Construct the 128-bit UUID:
        // Bits layout: [60-bit timestamp][4-bit version][2-bit variant][62-bit random]
        let uuid_val: u128 = ((timestamp as u128) << 68)
            | (7u128 << 64)
            | (0b10u128 << 62)
            | (random62 as u128);

        Uuid7(uuid_val)
    }

    /// Returns the inner `u128` representation.
    pub fn as_u128(&self) -> u128 {
        self.0
    }
}

impl fmt::Display for Uuid7 {
    /// Formats the UUIDv7 in the canonical form: 8-4-4-4-12 hexadecimal digits.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex_str = format!("{:032x}", self.0);
        write!(
            f,
            "{}-{}-{}-{}-{}",
            &hex_str[0..8],
            &hex_str[8..12],
            &hex_str[12..16],
            &hex_str[16..20],
            &hex_str[20..32]
        )
    }
}

/// Error type returned when parsing a UUIDv7 from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseUuid7Error(pub String);

impl fmt::Display for ParseUuid7Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseUuid7Error: {}", self.0)
    }
}

impl std::error::Error for ParseUuid7Error {}

impl FromStr for Uuid7 {
    type Err = ParseUuid7Error;

    /// Parses a UUIDv7 from its canonical string representation.
    ///
    /// This method validates:
    /// - The overall length and dash positions.
    /// - That the version field is 7.
    /// - That the variant field has its two most significant bits equal to binary `10`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove dashes.
        let hex: String = s.chars().filter(|&c| c != '-').collect();
        if hex.len() != 32 {
            return Err(ParseUuid7Error("Invalid length; expected 32 hex digits".into()));
        }
        let uuid_val = u128::from_str_radix(&hex, 16)
            .map_err(|e| ParseUuid7Error(format!("Invalid hex: {}", e)))?;

        // Validate version: bits 64-67 should equal 7.
        let version = (uuid_val >> 64) & 0xF;
        if version != 7 {
            return Err(ParseUuid7Error(format!("Invalid version: expected 7, got {}", version)));
        }
        // Validate variant: bits 62-63 should equal binary 10.
        let variant = (uuid_val >> 62) & 0x3;
        if variant != 0b10 {
            return Err(ParseUuid7Error(format!(
                "Invalid variant: expected binary 10, got {:b}",
                variant
            )));
        }
        Ok(Uuid7(uuid_val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn check_canonical_format(s: &str) {
        // The canonical UUID string is 36 characters with dashes at positions 8, 13, 18, 23.
        assert_eq!(s.len(), 36);
        assert_eq!(s.chars().nth(8).unwrap(), '-');
        assert_eq!(s.chars().nth(13).unwrap(), '-');
        assert_eq!(s.chars().nth(18).unwrap(), '-');
        assert_eq!(s.chars().nth(23).unwrap(), '-');
    }

    #[test]
    fn test_uuid7_new() {
        let uuid = Uuid7::new();
        let s = uuid.to_string();
        check_canonical_format(&s);
        // Check version and variant.
        let version_digit = s.chars().nth(14).unwrap(); // position within third group (index 14 overall)
        assert_eq!(version_digit, '7');

        let variant_digit = s.chars().nth(19).unwrap(); // first char of the fourth group.
        // variant_digit should be one of '8', '9', 'a', or 'b' (in lowercase)
        assert!(matches!(variant_digit, '8' | '9' | 'a' | 'b'));
    }

    #[test]
    fn test_uuid7_from_str_valid() {
        // Create a valid UUIDv7 string using our generator.
        let uuid_orig = Uuid7::new();
        let s = uuid_orig.to_string();
        let uuid_parsed = Uuid7::from_str(&s).expect("Parsing should succeed");
        assert_eq!(uuid_orig, uuid_parsed);
    }

    #[test]
    fn test_uuid7_from_str_invalid_length() {
        let s = "1234";
        let err = Uuid7::from_str(s).unwrap_err();
        assert!(err.0.contains("Invalid length"));
    }

    #[test]
    fn test_uuid7_from_str_invalid_version() {
        // Modify a valid UUID string to have an incorrect version digit.
        let mut s = Uuid7::new().to_string();
        // Replace version digit (index 14) with '1'
        s.replace_range(14..15, "1");
        let err = Uuid7::from_str(&s).unwrap_err();
        assert!(err.0.contains("Invalid version"));
    }

    #[test]
    fn test_uuid7_uniqueness() {
        let uuid1 = Uuid7::new();
        let uuid2 = Uuid7::new();
        assert_ne!(uuid1, uuid2);
    }
}
