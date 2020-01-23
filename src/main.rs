use structopt::StructOpt;
use structopt::clap::Shell;

mod cli;
mod config;

fn main() {
    // generate bash completions
    cli::LaunchArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

    // parse command line args (if passed in)
    let opt = cli::LaunchArgs::from_args();
    println!("{:?}", opt);

    // parse config file
    let cfg: config::Config = match confy::load("autoproxy") {
        Ok(data) => data,
        Err(_) => config::Config::default(),
    };
    println!("{:?}", cfg);
}
