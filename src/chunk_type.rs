use std::str::from_utf8;
// Implementation of Chunk Type section of PNG spec[http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html]
use std::{str::FromStr};
use std::convert::TryInto;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct ChunkType {
    bytes: [u8; 4]
}

#[derive(Debug)]
pub enum ChunkTypeErr {
    InvalidArgs
}

impl ChunkType {
    pub fn new(_bytes: &[u8; 4]) -> ChunkType {
        ChunkType { bytes: *_bytes }
    }

    // @desc Returns Type Bytes
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    // @desc Checks if reserved bit is valid
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    // @desc Check if chunk type is critical
    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }

    pub fn to_string(&self) -> String {
        String::from(from_utf8(&self.bytes).unwrap())
    }

}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeErr;

    fn try_from(bytes: [u8; 4]) -> Result<Self, ChunkTypeErr> {
        Ok(ChunkType { bytes })
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeErr;

    fn from_str(s: &str) -> Result<Self, ChunkTypeErr> {
        if s.chars().count() != 4 {
            return Err(ChunkTypeErr::InvalidArgs);
        } else {
            let temp = ChunkType { bytes: s.as_bytes().try_into().unwrap() };
            match temp.bytes.iter().all(|c| c.is_ascii_alphabetic()) {
                true => Ok(temp),
                _    => Err(ChunkTypeErr::InvalidArgs)
            }
        }
    }
}

impl fmt::Display for ChunkType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}"
            , self.bytes[0]
            , self.bytes[1]
            , self.bytes[2]
            , self.bytes[3])
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