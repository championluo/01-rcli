use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    //cargo run -- base64 encode -i abc
    #[command(name = "encode", about = "Encode base64 strings")]
    Encode(Base65EncodeOpts),
    //cargo run -- base64 decode -o abc
    #[command(name = "decode", about = "Decode base64 strings")]
    Decode(Base65DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base65EncodeOpts {
    #[arg(short, long)]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base65DecodeOpts {
    #[arg(short, long)]
    pub output: String,
}
