use crate::{process_http_serve, valid_path, CmdExecutor};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    //cargo run -- text sign -i abc
    #[command(about = "Serve a directory as a http server")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = valid_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // println!("{:?}", opts);
        // println!("Serving at http://0.0.0.0:{}", opts.port);\

        //因为process_http_serve已经是个async函数，所以这里要await,而要使用await，main函数需要是async的
        //同时使用tokio main 来包装 main函数
        process_http_serve(self.dir, self.port).await
    }
}

// impl CmdExecutor for HttpSubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             HttpSubCommand::Serve(opts) => opts.execute().await,
//         }
//     }
// }
