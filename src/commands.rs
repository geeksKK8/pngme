use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};
use crate::{Error, Result};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let file = fs::read(&args.file_path)?;
    let mut png = Png::try_from(file.as_slice())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
    let data = args.message.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, data);
    png.append_chunk(chunk);
    fs::write(args.file_path, png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let file = fs::read(&args.file_path)?;
    let png = Png::try_from(file.as_slice())?;
    let chunk = png.chunk_by_type(args.chunk_type.as_str()).unwrap();
    println!("{}", chunk.data_as_string()?);
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let file = fs::read(&args.file_path)?;
    let mut png = Png::try_from(file.as_slice())?;
    png.remove_chunk(args.chunk_type.as_str())?;
    fs::write(args.file_path, png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let file = fs::read(&args.file_path)?;
    let png = Png::try_from(file.as_slice())?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
