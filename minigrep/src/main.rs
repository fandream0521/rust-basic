use std::{env, process};

use minigrep::{run, Config};
fn main() {
    let args = env::args();
    let config = Config::build(args).unwrap();
    if let Err(e) = run(config) {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}
