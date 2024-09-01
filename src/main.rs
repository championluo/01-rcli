//rcli csv -i input.csv -o output.json --header -d ','
use anyhow::Result;
use clap::Parser;
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> Result<()> {
    //添加日志功能
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    //改完这里后
    //使用 cargo build
    // 通过后在跑一下所有的test  cargo nextext run
    // git status
    // cargo install --path .
    // 当执行 cargo install --path . 时，cargo 会执行以下操作：
    //编译当前目录下的 Rust 包。
    //如果包中有定义了 bin 类型的可执行二进制文件（在 Cargo.toml 文件中声明），则将这些可执行文件安装到全局路径中。
    //在执行一条命令，验证代码是否通过，比如： rcli genpass -l 32
    opts.cmd.execute().await
}
