use std::net::TcpStream;
use structopt::StructOpt;
use structopt::clap::AppSettings;
use serde_derive::{Deserialize, Serialize};

pub const APP_NAME: &'static str = "autoproxy";

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
    #[structopt(name = "status")]
    Status {},
    /// Enables the autoproxy
    #[structopt(name = "enable")]
    Enable {},
    /// Disables the autoproxy
    #[structopt(name = "disable")]
    Disable {},
    /// List all proxy configurations and their status
    #[structopt(name = "list")]
    List {},
    /// Adds a new proxy configuration
    #[structopt(name = "add")]
    Add(Proxy),
    /// Removes an existing proxy configuration
    #[structopt(name = "remove")]
    Remove {
        /// Proxy configuration alias
        name: String,
    },
}

// config structs
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    enabled: Option<bool>,
    proxy: Option<Vec<Proxy>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Proxy {
    /// Alias for this proxy configuration
    pub name: String,
    /// HTTP proxy endpoint
    #[structopt(long)]
    pub http: Option<String>,
    /// HTTPS proxy endpoint
    #[structopt(long)]
    pub https: Option<String>,
    /// Comma-separated list of domain exceptions
    /// (i.e. "a.com, b.com")
    #[structopt(long)]
    pub no: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enabled: Some(false),
            proxy: None,
        }
    }
}

impl Config {
    pub fn status (&self, _verbosity: u8) {
        //todo: print status (enabled, disabled)
        //todo: let user know if proxy varibles are currently set
        //todo?: show status of proxy connections? - might be better in it's own subcommand.
    }

    pub fn enable_proxy(&mut self, verbosity: u8) {
        self.enabled = Some(true);
        self.update_config(verbosity);
    }

    pub fn disable_proxy(&mut self, verbosity: u8) {
        self.enabled = Some(false);
        self.update_config(verbosity);
    }

    pub fn remove_proxy(&mut self, verbosity: u8, name: String) {
        match &mut self.proxy {
            // if there are no configs availabled, exit early.
            None => {
                println!("there are no proxy configurations available to remove");
                std::process::exit(1);
            },
            // if we have proxy data available...
            Some(data) => {
                // find the index of the config to remove...
                match data.iter().position(|x| x.name == name) {
                    None => {
                        // if we're unable to find the config with the specific name, exit.
                        println!("unable to find a proxy configuration with the name: {:?}", name);
                        std::process::exit(1);
                    },
                    Some(idx) => {
                        // remove the entry, and save the config.
                        data.remove(idx);
                        self.update_config(verbosity);
                    }
                }
            }
        };
    }

    pub fn add_proxy(&mut self, verbosity: u8, proxy: Proxy) {
        // add a new proxy
        match &mut self.proxy {
            None => {
                // create a new list if none exists
                let mut new_list: Vec<Proxy> = Vec::<Proxy>::new();
                new_list.push(proxy);

                self.proxy = Some(new_list);
            },
            Some(data) => {
                data.push(proxy);
            }
        }

        // update config file
        self.update_config(verbosity);
        if verbosity > 0 {
            println!("proxy added to configuration");
        }
    }

    pub fn list_proxies(&self, verbosity: u8) {
        if verbosity > 1 {
            println!("listing available proxies")
        }

        let proxies: Vec<Proxy> = match &self.proxy {
            Some(data) => data.to_vec(),
            None => {
                println!("there are no proxy configurations available");
                std::process::exit(0);
            },
        };

        println!("---------------");
        for proxy in proxies.iter() {
            println!("proxy:\t{}", proxy.name);
            println!("-----");
            match &proxy.https {
                Some(data) => println!("https:\t{}", data),
                None => println!("https:\tnone"),
            }
            match &proxy.http {
                Some(data) => println!("http:\t{}", data),
                None => println!("http:\tnone"),
            }
            match &proxy.no {
                Some(data) => println!("no:\t{}", data),
                None => println!("no:\tnone"),
            }
            println!("---------------");
        }
    }

    pub fn determine_proxy(&self, verbosity: u8) {
        if let Ok(_) = TcpStream::connect("google.com:443") {
            if verbosity > 0 {
                println!("Connected to the server!");
            }
        } else {
            println!("Couldn't connect to server...");
        }
        std::process::exit(0)   
    }
    
    fn update_config(&self, verbosity: u8) {
        if verbosity > 1 {
            println!("writing config to filesystem");
        }
        match confy::store(APP_NAME, self) {
            Ok(_) => (),
            Err(err) => {
                println!("Error writing to config file: {:?}", err);
                std::process::exit(2)
            },
        };
    }
}