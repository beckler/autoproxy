extern crate confy;
extern crate structopt;

use structopt::StructOpt;

mod cli;
mod config;

fn main() {
    let opt = cli::LaunchArgs::from_args();
    println!("{:?}", opt);

    let cfg: config::Config = match confy::load("autoproxy") {
        Ok(data) => data,
        Err(_) => config::Config::default(),
    };
    println!("{:?}", cfg);
}
