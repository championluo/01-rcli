//首先有个lib.rs, 用来引入各种module
mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::{process_csv, process_genpass};
// pub use process::{process_csv};
