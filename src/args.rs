use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct PngMeArgs {
    #[clap(subcommand)]
    pub action_type: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    Encode(EncodePng),
    Decode(DecodePng),
    Remove(RemovePng),
    Print(PrintPng),
}

#[derive(Debug, Args)]
pub struct EncodePng {
    pub filename: String,
    pub chunk_type: String,
    pub message: String,
}

#[derive(Debug, Args)]
pub struct DecodePng {
    pub filename: String,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemovePng {
    pub filename: String,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintPng {
    pub filename: String,
}
