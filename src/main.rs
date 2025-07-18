use minigrep::Config;
use std::env;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(why) = minigrep::run(config) {
        eprintln!("Application error: {}", why);
        std::process::exit(1);
    }
}
