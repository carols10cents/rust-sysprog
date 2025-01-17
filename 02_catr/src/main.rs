extern crate catr;
use std::process;

fn main() {
    let config = match catr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = catr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
