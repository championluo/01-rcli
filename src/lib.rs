//首先有个lib.rs, 用来引入各种module
// mod opts;
mod cli;
mod process;

pub use cli::{Opts, SubCommand};
pub use process::{process_csv, process_genpass};
