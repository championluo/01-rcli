//rcli csv -i input.csv -o output.json --header -d ','

use anyhow::Result;
use clap::Parser;
// use rcli::{process_csv, Opts, SubCommand};
use rcli::{
    base64_decode, base64_encode, process_csv, process_genpass, Base64SubCommand, Opts, SubCommand,
    TextSubCommand,
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
                println!("{:?}", opts)
            }
            TextSubCommand::Verify(opts) => {
                println!("{:?}", opts)
            }
        },
    }
    Ok(())
}
