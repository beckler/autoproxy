use structopt::StructOpt;
use structopt::clap::AppSettings;

#[derive(StructOpt, Debug)]
#[structopt(about, global_settings = &[AppSettings::DisableVersion])]
pub struct LaunchArgs{
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
    /// Enables the autoproxy
    #[structopt(name = "enable")]
    Enable {},

    /// Disables the autoproxy
    #[structopt(name = "disable")]
    Disable {},

    /// List all proxy configurations and their status
    #[structopt(name = "list")]
    List {
        /// Prints results as json
        #[structopt(short, long)]
        json: bool,
    },

    /// Adds a new proxy configuration
    #[structopt(name = "add")]
    Add {
        /// Alias for this proxy configuration
        name: String,

        /// HTTP proxy endpoint
        #[structopt(long)]
        http: Option<String>,

        /// HTTPS proxy endpoint
        #[structopt(long)]
        https: Option<String>,

        /// Comma-separated list of domain exceptions
        /// (i.e. "a.com, b.com")
        #[structopt(long)]
        no: Option<String>
    },

    /// Removes an existing proxy configuration
    #[structopt(name = "remove")]
    Remove {
        /// Proxy configuration alias
        name: String,
    },
}