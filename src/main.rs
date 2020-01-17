use structopt::StructOpt;

mod cli;
mod config;

fn main() {
    let opt = cli::LaunchArgs::from_args();
    println!("{:?}", opt);

    let config = config::parse_config(None);
    println!("{:?}", config)
}

