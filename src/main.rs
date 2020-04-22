//! **ls-tiny** is a less functional `ls` command
//! it is somewhat richer than when no argument to the ls command is specified. (Because it's a little colored.)

use std::process;
use ls_tiny::Config;

fn main() {
    let mut config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("try: ls-tiny -h");
            process::exit(1);
        }
    };

    config.run();
}  