extern crate getopts;

use std::{env, process};
use getopts::Options;

fn main() {
    //argsを取得
    let args:Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    // optionの設定
    opts.optopt("n", "", "set number of lines", "");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        
    }
}


