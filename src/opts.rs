// use anyhow::{Error, Ok};
use clap::{Parser, Subcommand};
// use serde_json::from_str;
// use std::{any, path::Path, process::Output, str::FromStr};
use std::{path::Path, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = valid_input_path)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = false)]
    header: bool,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}

// fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
// match format.to_lowercase().as_str() {
//     "json" => Ok(OutputFormat::JSON),
//     "yaml" => Ok(OutputFormat::YAML),
//     "toml" => Ok(OutputFormat::TOML),
//     _ => Err("Invalid format"), //注意match的写法， 最后的default 使用 _ => Err("Invalid format")
// }

// }

//parse_format 方法改成调用FromStr的trait
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    //<OutputFormat>() 可以省略， 会自动推导类型
    // format.parse::<OutputFormat>()
    format.parse()
}

//其他类型转成字符串
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

//字符串转其他类型
// impl TryFrom<&str> for OutputFormat {
//     type Error = anyhow::Error;
//     // type Error = &'static str;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value.to_lowercase().as_str() {
//             "json" => Ok(OutputFormat::JSON),
//             "yaml" => Ok(OutputFormat::YAML),
//             "toml" => Ok(OutputFormat::TOML),
//             // _ => Err("Invalid format"),
//             v => anyhow::bail!("Unsupported format: {}", v),
//         }
//     }
// }

// impl TryFrom<&str> 如果要用Parser的话，最好改写成FromStr的trait
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

#[derive(Debug, Parser)] //相当于注解，打上标识的结构体再特定场景下会有特别处理
#[command{name="rcli", version, author, about, long_about = None}]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)] // Opts中实现了Debug trait， 所以类型属性中也应该实现Debug， 不然无法进行debug输出
pub enum SubCommand {
    #[command(name = "csv", about = "Show Csv, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

//出参还可以是静态类型， 静态类型就是直接使用方法区中字符串的字面值常量，其生命周期和应用存活期间一致
fn valid_input_path(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        //into就是filename进去堆空间
        Ok(filename.into())
    } else {
        // Err(format!("file not found", filename))
        //这里into函数可以简化
        // Err("file not found".into())
        Err("file not found")
    }
}
