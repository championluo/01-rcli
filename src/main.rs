//rcli csv -i input.csv -o output.json --header -d ','

use anyhow::Result;
use clap::Parser;
// use rcli::{process_csv, Opts, SubCommand};
use rcli::{process_csv, process_genpass, Base64SubCommand, Opts, SubCommand};

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
        // cargo run -- base64
        SubCommand::Base64(subcmd) => {
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    // let encoded = base64::encode(&opts.input);
                    println!("{:?}", opts);
                }
                Base64SubCommand::Decode(opts) => {
                    // let decoded = base64::decode(&opts.input)?;
                    // println!("{:?}", decoded);
                    println!("{:?}", opts)
                }
            }
        }
    }
    Ok(())
}
