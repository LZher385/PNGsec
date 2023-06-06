mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod util;

use args::PngMeArgs;
use clap::Parser;

use crate::args::ActionType;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngMeArgs::parse();
    match args.action_type {
        ActionType::Encode(args) => commands::encode(args),
        ActionType::Decode(args) => commands::decode(args),
        ActionType::Remove(args) => commands::remove(args),
        ActionType::Print(args) => commands::print_chunks(args),
    }
}
