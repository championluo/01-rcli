//rcli csv -i input.csv -o output.json --header -d ','

use anyhow::Result;
use clap::Parser;
use rcli::{
    base64_decode, base64_encode, process_csv, process_genpass, process_sign, process_text_verify,
    Base64SubCommand, Opts, SubCommand, TextSubCommand,
};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        //cargo run --csv -i assert/juventus.csv -f yaml
        SubCommand::Csv(opts) => {
            //条件赋值+利用if let 解构
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                //这里默认json不太好
                // "output.json".into()
                format!("output.{}", opts.format) // 如果要是引用能够被format!,就需要实现Display trait
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        //cargo run -- genpass -l 8 --lowercase --uppercase --number --symbol
        SubCommand::GenPass(opts) => {
            let _ = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            );
        }
        // cargo run -- base64 encode|decode
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                base64_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                base64_decode(&opts.output, opts.format)?;
            }
        },
        // cargo run -- text sign|verify
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                //注意这里加签需要区分format类型
                //测试命令 cargo run -- text sign -k fixtures/blake3.txt
                process_sign(&opts.input, &opts.key, opts.format)?;
            }
            TextSubCommand::Verify(opts) => {
                println!("{:?}", opts);
                //测试命令 cargo run -- text verify -k fixtures/blake3.txt --sig 4dynUr9DyxEt8EjPi0OF1lHyPmCB_et6_Fty6hmmqjI
                process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
            }
        },
    }
    Ok(())
}
