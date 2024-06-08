use clap::{Parser, Subcommand};

use super::valid_input_path;

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
    //default_value = "-" 代表的意思是默认从标准输入中获取
    #[arg(short, long, value_parser = valid_input_path, default_value = "-")]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base65DecodeOpts {
    //default_value = "-" 代表的意思是默认从标准输入中获取
    #[arg(short, long, value_parser = valid_input_path, default_value = "-")]
    pub output: String,
}
