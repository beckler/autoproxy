use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "autoproxy", about)]
pub struct LaunchArgs{
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Config file
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Creates a new config file
    #[structopt(name = "init")]
    Init {},

    /// Enables the autoproxy
    #[structopt(name = "enable")]
    Enable {},

    /// Disables the autoproxy
    #[structopt(name = "disable")]
    Disable {},

    /// List all proxies in the current configuration the current status
    #[structopt(name = "list")]
    List {},
}