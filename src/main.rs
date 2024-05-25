//rcli csv -i input.csv -o output.json --header -d ','

use std::path::Path;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] //相当于注解，打上标识的结构体再特定场景下会有特别处理
#[command{name="rcli", version, author, about, long_about = None}]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)] // Opts中实现了Debug trait， 所以类型属性中也应该实现Debug， 不然无法进行debug输出
enum SubCommand {
    #[command(name = "csv", about = "Show Csv, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser= valid_input_path)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = false)]
    header: bool,
}

fn main() {
    // println!("Hello, world!");
    let opts = Opts::parse();
    println!("{:?}", opts);
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
