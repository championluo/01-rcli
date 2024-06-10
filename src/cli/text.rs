use super::valid_path;
use clap::{Parser, Subcommand};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Subcommand)]
pub enum TextSubCommand {
    //cargo run -- text sign -i abc
    #[command(about = "Sign a message with private/shared key")]
    Sign(TextSignOpts),
    //cargo run -- text verify -o abc
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    //default_value = "-" 代表的意思是默认从标准输入中获取
    #[arg(short, long, value_parser = valid_path, default_value = "-")]
    pub input: String,

    //这里不用加 , default_value = "-"
    //因为 input 已经默认了从标准输入输入，这里再默认标准输入就会冲突
    #[arg(short, long, value_parser = valid_path)]
    pub key: String,

    #[arg(long,default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    //default_value = "-" 代表的意思是默认从标准输入中获取
    #[arg(short, long, value_parser = valid_path, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = valid_path)]
    pub key: String,

    #[arg(long,default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,

    #[arg(short, long)]
    pub sig: String,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "BLAKE3",
            TextSignFormat::Ed25519 => "ED25519",
        }
    }
}

//into函数会调用 from函数
impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
