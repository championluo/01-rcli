//首先有个lib.rs, 用来引入各种module
// mod opts;
mod cli;
mod process;
mod utils;

pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand, CsvOpts, GenPassOpts, HttpServeOpts,
    HttpSubCommand, Opts, OutputFormat, SubCommand, TextKeyGenerateOpts, TextSignFormat,
    TextSignOpts, TextSubCommand, TextVerifyOpts,
};
use enum_dispatch::enum_dispatch;
pub use process::{
    base64_decode, base64_encode, process_csv, process_genpass, process_http_serve, process_sign,
    process_text_generate, process_text_verify,
};
pub use utils::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
