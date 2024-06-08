mod base64;
mod csv;
mod genpass;

use clap::{Parser, Subcommand};

use self::csv::CsvOpts;
pub use self::{base64::*, csv::OutputFormat};
use genpass::GenPassOpts;

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
    //1 先生成一个新的子命令
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),

    // #[command(name = "base64", about = "Encode or decode base64 strings")]
    //这样写会报错： the trait bound `Base64SubCommand: clap::Args` is not satisfied the following other types implement trait `clap::Args`:
    //这是因为Base64SubCommand 是 Base64的子命令，Base64下面又分化了2个子命令，所以这里要标注command成subcommand
    #[command(subcommand)]
    Base64(Base64SubCommand),
}
