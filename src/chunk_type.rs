use crate::util::get_bit_value;
use std::fmt;
use std::str::{from_utf8, FromStr};

use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    value: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.value
    }

    pub fn is_critical(&self) -> bool {
        get_bit_value(&self.value[0], 5) == 0
    }

    pub fn is_public(&self) -> bool {
        get_bit_value(&self.value[1], 5) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        get_bit_value(&self.value[2], 5) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        get_bit_value(&self.value[3], 5) == 1
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii() && (byte.is_ascii_lowercase() || byte.is_ascii_uppercase())
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && self.value.iter().all(|x| ChunkType::is_valid_byte(*x))
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Error> {
        Ok(ChunkType { value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if s.len() != 4 {
            return Err(Error::from("Invalid length for string"));
        }
        let bytes = s.as_bytes();
        from_utf8(bytes)?;
        let mut value: [u8; 4] = [0; 4];

        for (i, &byte) in bytes.iter().enumerate() {
            if !ChunkType::is_valid_byte(byte) {
                return Err(Error::from("Byte is not valid"));
            }
            value[i] = byte;
        }
        Ok(ChunkType { value })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.value).map_err(|_| fmt::Error)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
