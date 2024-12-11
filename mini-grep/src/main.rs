use mini_grep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Could not parse CLI arguments: {err}");
        std::process::exit(1);
    });
    //run(config).expect("could not run the program");

    if let Err(err) = mini_grep::run(config) {
        eprintln!("Could not run application: {err}");
        process::exit(1);
    }
}
