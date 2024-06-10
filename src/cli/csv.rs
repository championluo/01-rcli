use clap::Parser;
use std::{fmt, str::FromStr};

use super::valid_path; //注意这个super

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = valid_path)]
    pub input: String,

    // #[arg(short, long, default_value = "output.json")]
    #[arg(short, long)] //output不是固定的了（json， yaml等）， 所以这里将output设置成Option
    pub output: Option<String>,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = false)]
    header: bool,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}

//parse_format 方法改成调用FromStr的trait
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    //<OutputFormat>() 可以省略， 会自动推导类型
    // format.parse::<OutputFormat>()
    format.parse()
}

// //出参还可以是静态类型， 静态类型就是直接使用方法区中字符串的字面值常量，其生命周期和应用存活期间一致
// fn valid_input_path(filename: &str) -> Result<String, &'static str> {
//     if Path::new(filename).exists() {
//         //into就是filename进去堆空间
//         Ok(filename.into())
//     } else {
//         // Err(format!("file not found", filename))
//         //这里into函数可以简化
//         // Err("file not found".into())
//         Err("file not found")
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
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

impl fmt::Display for OutputFormat {
    /**
     * 大模型的解释
     * fmt 函数是 Display trait 需要实现的核心方法，它接收两个参数：
     *  &self：表示对当前 OutputFormat 实例的引用，允许我们在不转移所有权的情况下访问其实例数据。
     *  f: &mut fmt::Formatter<'_>：是一个可变引用到 Formatter 对象，它携带了关于如何格式化输出的信息，
     *       包括缓冲区、对齐方式、宽度等。'_ 是一个生命周期标注，表明 Formatter 的生命周期与调用者相同。
     * 函数的返回类型是 fmt::Result，这是 Rust 标准库中对格式化操作成功或失败的标准返回类型，
     *  通常表示为 Ok(()) 表示成功，或 Err(fmt::Error) 表示失败。
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这行代码使用 write! 宏向 f（即 Formatter 的实例）写入格式化的字符串。
        // 它的工作原理类似于 println!，但不换行，并且目标是 Formatter 而不是标准输出。

        //Into::<&'static str>::into(*self)：这部分代码将 self（即 OutputFormat 的实例）
        // 转换为一个 &'static str 类型的引用。into会调用from函数，而我们已经为OutputFormat实现了From
        //——  *self解引用 self（因为 self 是引用类型），然后通过 Into trait 转换为指定类型。

        //最后，write! 宏内部会处理格式化字符串和变量的插入，确保输出到 f 的内容符合预期，
        //并且返回一个 fmt::Result 表明操作是否成功。
        // write!(f,"{}",Into::<&'static str>::into(*self))

        //&'static可以不要， 因为不关心str的生命周期，使用完即止
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
