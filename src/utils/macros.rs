// Colors for macros come from: https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
macro_rules! log {
    ($t:expr) => {
        println!("\x1b[96mlog\x1b[0m: {}", $t)
    };
}
pub(crate) use log;

macro_rules! warn_ {
    ($w:expr, $t:expr) => {
        println!("\x1b[93mwarning {}\x1b[0m: {}", $w, $t)
    };
}
pub(crate) use warn_;

macro_rules! warn_unimplemented {
    ($t:expr) => {
        println!("\x1b[94munimplemented\x1b[0m: {}", $t)
    };
}
pub(crate) use warn_unimplemented;
