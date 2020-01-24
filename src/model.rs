use structopt::StructOpt;
use structopt::clap::AppSettings;
use serde_derive::{Deserialize, Serialize};

// cli structs
#[derive(StructOpt, Debug)]
#[structopt(about, global_settings = &[AppSettings::DisableVersion])]
pub struct LaunchArgs{
    // /// Activate debug mode
    // #[structopt(short, long)]
    // debug: bool,
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
    // subcommand
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
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
    Add(ProxyType),
    /// Removes an existing proxy configuration
    #[structopt(name = "remove")]
    Remove {
        /// Proxy configuration alias
        name: String,
    },
}

#[derive(StructOpt, Debug)]
pub enum ProxyType {
    #[structopt(name = "proxy")]
    Proxy(Proxy),
    #[structopt(name = "pac")]
    ProxyAutoConfig(ProxyAutoConfig),
}

// config structs
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    enabled: Option<bool>,
    proxy: Option<Vec<Proxy>>,
}

#[derive(Debug, Serialize, Deserialize, StructOpt)]
pub struct Proxy {
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
    no: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, StructOpt)]
pub struct ProxyAutoConfig {
    /// Alias for this proxy configuration
    name: String,
    /// URL to PAC file
    pac: String,
    /// URL to evaluate against the PAC file when loaded
    #[structopt(long, default_value = "https://google.com")]
    test_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enabled: Some(false),
            proxy: Some(vec![Proxy {
                name: String::new(),
                http: None,
                https: None,
                no: None,
            }])
        }
    }
}