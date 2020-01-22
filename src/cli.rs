extern crate structopt;

// use std::path::PathBuf;
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

    /// Adds a new proxy configuration
    #[structopt(name = "add")]
    Add {
        name: String,
        #[structopt(long, about = "Define the http proxy endpoint")]
        http: Option<String>,

        #[structopt(long, about = "Define the https proxy endpoint")]
        https: Option<String>,

        #[structopt(long, about = "Define endpoints that should be excepted from the proxy")]
        no: Option<String>
    },

    /// Removes an existing proxy configuration
    #[structopt(name = "remove")]
    Remove {},
}