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
//! .expect("Failed to parse UUIDv7");
//! ```

use rand::Rng;
use std::fmt;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

/// A UUIDv7 value.
///
/// Internally stored as 16 bytes, following the UUIDv7 specification.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Uuid7 {
    /// The 16 bytes that make up this UUID
    bytes: [u8; 16],
}

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
        Self::new_with_rng(&mut rand::rng())
    }

    /// Generates a new UUIDv7 using a custom random number generator.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rand::Rng;
    /// use leachuuid7::Uuid7;
    ///
    /// // Example of using with a custom RNG
    /// let mut rng = rand::thread_rng();
    /// let uuid = Uuid7::new_with_rng(&mut rng);
    /// println!("{}", uuid);
    /// ```
    pub fn new_with_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut bytes = [0u8; 16];
        
        // Get timestamp as milliseconds since epoch
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let millis = now.as_millis() as u64;
        
        // First 48 bits: timestamp (6 bytes)
        bytes[0] = (millis >> 40) as u8;
        bytes[1] = (millis >> 32) as u8;
        bytes[2] = (millis >> 24) as u8;
        bytes[3] = (millis >> 16) as u8;
        bytes[4] = (millis >> 8) as u8;
        bytes[5] = millis as u8;
        
        // Fill remaining bytes with random data
        rng.fill_bytes(&mut bytes[6..]);
        
        // Set version (7) in the most significant 4 bits of the 7th byte
        bytes[6] = (bytes[6] & 0x0F) | 0x70;
        
        // Set variant (binary 10xx) in the most significant 2 bits of the 9th byte
        bytes[8] = (bytes[8] & 0x3F) | 0x80;
        
        Self { bytes }
    }

    /// Returns the inner byte representation.
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    /// Returns the value as a u128.
    pub fn as_u128(&self) -> u128 {
        let mut value: u128 = 0;
        for (i, &byte) in self.bytes.iter().enumerate() {
            value |= (byte as u128) << (120 - (i * 8));
        }
        value
    }
}

impl fmt::Display for Uuid7 {
    /// Formats the UUIDv7 in the canonical form: 8-4-4-4-12 hexadecimal digits.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3],
            self.bytes[4], self.bytes[5],
            self.bytes[6], self.bytes[7],
            self.bytes[8], self.bytes[9],
            self.bytes[10], self.bytes[11], self.bytes[12], self.bytes[13], self.bytes[14], self.bytes[15]
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
        // Check that the string has the correct length
        if s.len() != 36 {
            return Err(ParseUuid7Error("Invalid UUID length; expected 36 characters".into()));
        }
        
        // Check dash positions
        if s.chars().nth(8) != Some('-') || s.chars().nth(13) != Some('-') || 
           s.chars().nth(18) != Some('-') || s.chars().nth(23) != Some('-') {
            return Err(ParseUuid7Error("Invalid UUID format; expected dashes at positions 8, 13, 18, and 23".into()));
        }

        // Check version (digit at position 14, should be 7)
        if s.chars().nth(14) != Some('7') {
            return Err(ParseUuid7Error(format!(
                "Invalid version: expected 7, got {}",
                s.chars().nth(14).unwrap_or('?')
            )));
        }

        // Check variant (digit at position 19, should be 8, 9, a, or b)
        let variant_char = s.chars().nth(19).unwrap_or('?');
        if !matches!(variant_char, '8' | '9' | 'a' | 'b' | 'A' | 'B') {
            return Err(ParseUuid7Error(format!(
                "Invalid variant: expected one of 8, 9, a, b, got {}",
                variant_char
            )));
        }

        // Remove dashes and parse the hex string
        let hex: String = s.chars().filter(|&c| c != '-').collect();
        
        // Parse hex string into bytes
        let mut bytes = [0u8; 16];
        for i in 0..16 {
            let byte_str = &hex[i*2..i*2+2];
            bytes[i] = u8::from_str_radix(byte_str, 16)
                .map_err(|_| ParseUuid7Error(format!("Invalid hex at position {}: {}", i, byte_str)))?;
        }
        
        Ok(Uuid7 { bytes })
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
        println!("Generated UUID: {}", s);
        
        check_canonical_format(&s);

        // The UUID standard specifies:
        // xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx
        // where M is the version (7) and N is the variant (8, 9, a, or b)
        
        // Version character should be at position 14 (15th character)
        let version_char = s.chars().nth(14).unwrap();
        assert_eq!(version_char, '7', "Version character should be '7', found '{}' in UUID: {}", version_char, s);

        // Variant character should be at position 19 (20th character)
        let variant_char = s.chars().nth(19).unwrap();
        assert!(
            matches!(variant_char, '8' | '9' | 'a' | 'b'),
            "Variant character '{}' is not one of '8', '9', 'a', or 'b' in UUID: {}",
            variant_char,
            s
        );
    }

    #[test]
    fn test_uuid7_from_str_valid() {
        // Create a valid UUIDv7 string using our generator.
        let uuid_orig = Uuid7::new();
        let s = uuid_orig.to_string();
        println!("Testing parsing of valid UUID: {}", s);
        
        let uuid_parsed = Uuid7::from_str(&s).expect("Parsing should succeed");
        assert_eq!(uuid_orig, uuid_parsed);
    }

    #[test]
    fn test_uuid7_from_str_invalid_length() {
        let s = "1234";
        let err = Uuid7::from_str(s).unwrap_err();
        assert!(err.0.contains("Invalid"), "Error should mention invalid length");
    }

    #[test]
    fn test_uuid7_from_str_invalid_version() {
        // Create a new UUID with a valid format but with version 1 instead of 7
        let s = "01234567-89ab-1def-8123-456789abcdef";
        let err = Uuid7::from_str(s).unwrap_err();
        assert!(err.0.contains("Invalid version"), "Error should mention invalid version");
    }

    #[test]
    fn test_uuid7_uniqueness() {
        let uuid1 = Uuid7::new();
        let uuid2 = Uuid7::new();
        assert_ne!(uuid1, uuid2, "Two generated UUIDs should be different");
    }
    
    #[test]
    fn test_roundtrip_as_u128() {
        let uuid = Uuid7::new();
        let value = uuid.as_u128();
        let bytes = uuid.as_bytes();
        
        // Check that each byte is in the correct position
        for i in 0..16 {
            let expected = bytes[i];
            let actual = ((value >> (120 - (i * 8))) & 0xFF) as u8;
            assert_eq!(expected, actual, "Byte at position {} doesn't match", i);
        }
    }
}