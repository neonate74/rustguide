extern crate greprs;

use std::env::args;
use greprs::Config;

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::new(&args);

    greprs::run(config);
}