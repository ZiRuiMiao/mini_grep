use std::env;

use grep::Config;
use std::process;

fn main() {
    // println!("{:?}", args);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem arguments :{}", err);
        process::exit(1);
    });

    if let Err(e) = grep::run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    }
}
