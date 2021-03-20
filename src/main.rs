#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    let cfg_name = ObsArgs::new();
    println!("{}", cfg_name.cfg);
}

struct ObsArgs {
    cfg: String,
}

impl ObsArgs {
    fn new() -> Self {
        let param_cfg = Arg::with_name("cfg")
            .short("t")
            .long("rotate")
            .takes_value(true)
            .multiple(true)
            .value_name("FILE(S)")
            .help("Rotate specified list of files");

        let matches = App::new("obs")
            .version("v1.0-beta")
            .about("Rust implementation of the dotfile rotation util")
            .author(crate_authors!("\n"))
            .args(&[param_cfg])
            .get_matches();

        let name = matches.value_of("cfg").unwrap();

        ObsArgs {
            cfg: String::from(name),
        }
    }
}
