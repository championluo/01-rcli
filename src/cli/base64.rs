use std::{fmt::Display, str::FromStr};

use clap::{Parser, Subcommand};

use super::valid_file;

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    //cargo run -- base64 encode -i abc
    #[command(name = "encode", about = "Encode base64 strings")]
    Encode(Base64EncodeOpts),
    //cargo run -- base64 decode -o abc
    #[command(name = "decode", about = "Decode base64 strings")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    //default_value = "-" 代表的意思是默认从标准输入中获取
    #[arg(short, long, value_parser = valid_file, default_value = "-")]
    pub input: String,

    //cargo run -- base64 encode -i assets/meinvpic.jpeg --format urlsafe
    #[arg(long, value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    //default_value = "-" 代表的意思是默认从标准输入中获取
    #[arg(short, long, value_parser = valid_file, default_value = "-")]
    pub output: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "STANDARD",
            Base64Format::UrlSafe => "URL_SAFE",
        }
    }
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
