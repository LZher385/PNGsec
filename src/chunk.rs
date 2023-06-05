use std::{
    fmt,
    str::{from_utf8, Utf8Error},
};

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::{chunk_type::ChunkType, Error};

#[derive(Debug, Clone)]
struct Chunk {
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
}

impl Chunk {
    pub const CRC_INSTANCE: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    pub fn length(&self) -> u32 {
        self.chunk_data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        let mut crc_calc: Vec<u8> = Vec::new();
        crc_calc.extend_from_slice(self.chunk_type().bytes().as_ref());
        crc_calc.extend_from_slice(self.data().as_ref());
        println!("{}", from_utf8(crc_calc.as_ref()).unwrap());
        Chunk::CRC_INSTANCE.checksum(&crc_calc)
    }

    pub fn data_as_string(&self) -> Result<String, Utf8Error> {
        match from_utf8(self.chunk_data.as_ref()) {
            Ok(string) => Ok(string.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length_bytes = self.length().to_be_bytes();
        let chunk_type_bytes = self.chunk_type().bytes();
        let data_bytes = self.data();
        let crc_bytes = self.crc().to_be_bytes();
        length_bytes
            .iter()
            .chain(chunk_type_bytes.iter())
            .chain(data_bytes.iter())
            .chain(crc_bytes.iter())
            .copied()
            .collect()
    }

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Self {
            chunk_type,
            chunk_data: data,
        }
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let chunk_type_bytes: [u8; 4] = value[4..8].try_into()?;
        let chunk_type: ChunkType = chunk_type_bytes.try_into()?;
        let chunk_data = &value[8..value.len() - 4];
        let crc_bytes = &value[value.len() - 4..];
        let crc: u32 = u32::from_be_bytes(crc_bytes.try_into()?);

        let new = Chunk::new(chunk_type, chunk_data.into());
        println!("{}", new.crc());
        if new.crc() != crc {
            return Err(Error::from("CRC validation error"));
        }
        Ok(new)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
