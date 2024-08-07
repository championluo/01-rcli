//首先有个lib.rs, 用来引入各种module
// mod opts;
mod cli;
mod process;
mod utils;

pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand, Opts, SubCommand, TextSignFormat,
    TextSignOpts, TextSubCommand, TextVerifyOpts,
};
pub use process::{
    base64_decode, base64_encode, process_csv, process_genpass, process_sign,
    process_text_generate, process_text_verify,
};
pub use utils::*;
