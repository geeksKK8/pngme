use args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Cli::parse();
    match args.command {
        args::PngMeArgs::Encode(EncodeArgs{file_path, chunk_type, message}) => {
            commands::encode(EncodeArgs { file_path, chunk_type, message})
        }
        args::PngMeArgs::Decode(DecodeArgs{file_path, chunk_type}) => {
            commands::decode(DecodeArgs { file_path, chunk_type})
        }
        args::PngMeArgs::Remove(RemoveArgs{file_path, chunk_type}) => {
            commands::remove(RemoveArgs { file_path, chunk_type})
        }
        args::PngMeArgs::Print(PrintArgs{file_path}) => {
            commands::print_chunks(PrintArgs { file_path})
        }
    }
}
