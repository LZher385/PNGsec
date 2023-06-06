use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{DecodePng, EncodePng, PrintPng, RemovePng};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Error;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodePng) -> Result<(), Error> {
    let input = fs::read(&args.filename)?;
    let mut png = Png::try_from(&input[..])?;
    // Chunk::new should just accept &str and create ChunkType within the function itself
    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;
    let chunk = Chunk::new(
        ChunkType::try_from(chunk_type)?,
        args.message.as_bytes().to_vec(),
    );
    png.append_chunk(chunk);
    fs::write(args.filename, png.as_bytes());
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodePng) -> Result<(), Error> {
    let input = fs::read(&args.filename)?;
    let png = Png::try_from(&input[..])?;
    let chunk = png.chunk_by_type(&args.chunk_type);
    if let Some(c) = chunk {
        print!("{}", c);
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemovePng) -> Result<(), Error> {
    let input = fs::read(&args.filename)?;
    let mut png = Png::try_from(&input[..])?;
    match png.remove_chunk(&args.chunk_type) {
        Ok(chunk) => {
            fs::write(&args.filename, png.as_bytes())?;
        }
        Err(e) => print!("Error: {}", e),
    }
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintPng) -> Result<(), Error> {
    let input = fs::read(&args.filename)?;
    let png = Png::try_from(&input[..])?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
