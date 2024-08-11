use std::fs;

//rcli csv -i input.csv -o output.json --header -d ','
use anyhow::Result;
use clap::Parser;
use rcli::{
    base64_decode, base64_encode, process_csv, process_genpass, process_http_serve, process_sign,
    process_text_generate, process_text_verify, Base64SubCommand, HttpSubCommand, Opts, SubCommand,
    TextSubCommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> Result<()> {
    //添加日志功能
    tracing_subscriber::fmt::init();
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
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            //打印动作放到main中
            println!("{}", password);

            let estimate = zxcvbn(&password, &[]);
            //注意这里使用的是标准错误数据
            // cargo run -- genpass -l 16 > out.txt
            //这里打印到txt文件中只有上面的 标准输出, 这里的标准错误输出只会在终端显示
            eprintln!("Password strength: {}", estimate.score());
        }
        // cargo run -- base64 encode|decode
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encode = base64_encode(&opts.input, opts.format)?;
                //解码命令 cargo run -- base64 decode -o temp64.txt --format urlsafe
                //注意，这么写decode会报错 Error: Invalid symbol 10, offset 736.
                //cargo run -- base64 encode -i Cargo.toml --format urlsafe > temp64.txt
                // 使用上面这个命令， 会把 Encoded: 也输出到 文件中， 导致解码失败
                // println!("Encoded: {}", encode);
                println!("{}", encode);
            }
            Base64SubCommand::Decode(opts) => {
                let decode = base64_decode(&opts.output, opts.format)?;

                // let decode = URL_SAFE.decode(input)?;
                //TODO: decoded data might not be string (but for this example, we assume it is)
                let decode: String = String::from_utf8(decode)?;
                print!("{}", decode);
            }
        },
        // cargo run -- text sign|verify
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                //注意这里加签需要区分format类型
                //测试命令 cargo run -- text sign -k fixtures/blake3.txt
                let process_sign = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", process_sign);
            }
            TextSubCommand::Verify(opts) => {
                println!("{:?}", opts);
                //测试命令 cargo run -- text verify -k fixtures/blake3.txt --sig 4dynUr9DyxEt8EjPi0OF1lHyPmCB_et6_Fty6hmmqjI
                let verify_result =
                    process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verify_result);
            }
            TextSubCommand::Generate(opts) => {
                println!("{:?}", opts);
                let key = process_text_generate(opts.format)?;
                //这里要把结果能够输出到文件中
                //根据不同类型,把返回写到文件中
                match opts.format {
                    //cargo run -- text generate -o fixtures
                    //可以在fixtures文件夹中看到生成的blake3 的密钥
                    rcli::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    //cargo run -- text generate -o fixtures -f ed25519
                    //可以在fixtures文件夹中看到生成的私钥和公钥
                    rcli::TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        //cargo run -- http serve
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(opts) => {
                // println!("{:?}", opts);
                // println!("Serving at http://0.0.0.0:{}", opts.port);\

                //因为process_http_serve已经是个async函数，所以这里要await,而要使用await，main函数需要是async的
                //同时使用tokio main 来包装 main函数
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
