use crate::process_sign;
use crate::process_text_generate;
use crate::process_text_verify;
use crate::valid_file;
use crate::valid_path;
use crate::CmdExecutor;
use anyhow::Result;
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use std::{fmt::Display, fs, path::PathBuf, str::FromStr};

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    //cargo run -- text sign -i abc
    #[command(about = "Sign a message with private/shared key")]
    Sign(TextSignOpts),
    //cargo run -- text verify -o abc
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),

    #[command(about = "generate text key")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    //default_value = "-" 代表的意思是默认从标准输入中获取
    #[arg(short, long, value_parser = valid_file, default_value = "-")]
    pub input: String,

    //这里不用加 , default_value = "-"
    //因为 input 已经默认了从标准输入输入，这里再默认标准输入就会冲突
    #[arg(short, long, value_parser = valid_file)]
    pub key: String,

    #[arg(long,default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    //default_value = "-" 代表的意思是默认从标准输入中获取
    #[arg(short, long, value_parser = valid_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = valid_file)]
    pub key: String,

    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
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

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short,long,value_parser = valid_path)]
    pub output: PathBuf,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        //注意这里加签需要区分format类型
        //测试命令 cargo run -- text sign -k fixtures/blake3.txt
        let process_sign = process_sign(&self.input, &self.key, self.format)?;
        println!("{}", process_sign);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{:?}", self);
        //测试命令 cargo run -- text verify -k fixtures/blake3.txt --sig 4dynUr9DyxEt8EjPi0OF1lHyPmCB_et6_Fty6hmmqjI
        let verify_result = process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        println!("{}", verify_result);
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{:?}", self);
        let key = process_text_generate(self.format)?;

        //这里要把结果能够输出到文件中
        //根据不同类型,把返回写到文件中
        match self.format {
            //cargo run -- text generate -o fixtures
            //可以在fixtures文件夹中看到生成的blake3 的密钥
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0])?;
            }
            //cargo run -- text generate -o fixtures -f ed25519
            //可以在fixtures文件夹中看到生成的私钥和公钥
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0])?;
                fs::write(name.join("ed25519.pk"), &key[1])?;
            }
        }
        Ok(())
    }
}

// impl CmdExecutor for TextSubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             TextSubCommand::Sign(opts) => opts.execute().await,
//             TextSubCommand::Verify(opts) => opts.execute().await,
//             TextSubCommand::Generate(opts) => opts.execute().await,
//         }
//     }
// }
