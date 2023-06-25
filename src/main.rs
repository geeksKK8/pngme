use args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args = args::Cli::parse();
    match args.command {
        args::PngMeArgs::Encode(EncodeArgs{file_path, chunk_type, message}) => {
            println!("call encode: {:?}",EncodeArgs{file_path, chunk_type, message});
        }
        args::PngMeArgs::Decode(DecodeArgs{file_path, chunk_type}) => {
            println!("call decode");
        }
        args::PngMeArgs::Remove(RemoveArgs{file_path, chunk_type}) => {
            println!("call remove");
        }
        args::PngMeArgs::Print(PrintArgs{file_path}) => {
            println!("call print");
        }
        // args::PngMeArgs::Encode(encode_args) => commands::encode(
        //     encode_args.file_path,
        //     encode_args.chunk_type,
        //     encode_args.message,
        // ),
        // args::PngMeArgs::Decode(decode_args) => {
        //     commands::decode(decode_args.file_path, decode_args.chunk_type)
        // }
        // args::PngMeArgs::Remove(remove_args) => {
        //     commands::remove(remove_args.file_path, remove_args.chunk_type)
        // }
        // args::PngMeArgs::Print(print_args) => commands::print(print_args.file_path),
    }
}
