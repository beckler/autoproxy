use std::env;
use std::net::TcpStream;
use log::{info, debug, warn, error};
use console::style;
use structopt::StructOpt;
use serde_derive::{Deserialize, Serialize};

pub const APP_NAME: &'static str = "autoproxy";


// config structs
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    enabled: bool,
    proxy: Option<Vec<Proxy>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Proxy {
    /// Alias for this proxy configuration
    pub name: String,
    /// Domain:Port to test proxy connections.
    /// This tests the connection by creating a TCP connection.
    #[structopt(short, long, default_value="docs.tnybit.com:80")]
    pub test_url: String,
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
            enabled: false,
            proxy: None,
        }
    }
}

impl Config {
    // TODO: needs tests
    pub fn status (&self) {
        // enabled/disabled status
        println!("autoproxy is currently {}", if self.enabled { 
            style("enabled").green() 
        } else { 
            style("disabled").red() 
        });
        // proxy status
        print_status("https_proxy");
        print_status("HTTPS_PROXY");
        print_status("http_proxy");
        print_status("HTTP_PROXY");
        print_status("no_proxy");
        print_status("NO_PROXY");
    }

    // TODO: needs tests
    pub fn enable_proxy(&mut self) {
        debug!("autoproxy is being enabled");
        self.enabled = true;
        self.update_config();
    }

    // TODO: needs tests
    pub fn disable_proxy(&mut self) {
        debug!("autoproxy is being disabled");
        self.enabled = false;
        self.update_config();
    }

    // TODO: needs tests
    pub fn remove_proxy(&mut self, name: String) {
        match &mut self.proxy {
            // if there are no configs availabled, exit early.
            None => {
                warn!("there are no proxy configurations available to remove");
                std::process::exit(1);
            },
            // if we have proxy data available...
            Some(data) => {
                // find the index of the config to remove...
                match data.iter().position(|x| x.name == name) {
                    None => {
                        // if we're unable to find the config with the specific name, exit.
                        warn!("unable to find a proxy configuration with the name: {:?}", name);
                        std::process::exit(1);
                    },
                    Some(idx) => {
                        // remove the entry, and save the config.
                        debug!("removing entry from config file");
                        data.remove(idx);
                        self.update_config();
                    }
                }
            }
        };
        println!("removed proxy")
    }

    // TODO: needs tests
    pub fn add_proxy(&mut self, proxy: Proxy) {
        // add a new proxy
        match &mut self.proxy {
            None => {
                // create a new list if none exists
                debug!("config file is empty - added data to new config");
                let mut new_list: Vec<Proxy> = Vec::<Proxy>::new();
                new_list.push(proxy);

                self.proxy = Some(new_list);
            },
            Some(data) => {
                debug!("adding proxy to config file");
                data.push(proxy);
            }
        }

        // update config file
        self.update_config();
        println!("added proxy");
    }

    // TODO: needs tests
    pub fn list_proxies(&self) {
        debug!("listing all available proxies");
        let proxies: Vec<Proxy> = match &self.proxy {
            Some(data) => data.to_vec(),
            None => {
                warn!("there are no proxy configurations available");
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

    // TODO: needs tests
    pub fn determine_proxy(&self) {
        if let Ok(_) = TcpStream::connect("google.com:443") {
            info!("connected to the server");
        } else {
            info!("unable to connect to server");
        }
        std::process::exit(0)   
    }

    fn update_environment(&self) {
        
    }
    
    // TODO: needs tests
    fn update_config(&self) {
        debug!("writing or updating config to filesystem");
        match confy::store(APP_NAME, self) {
            Ok(_) => (),
            Err(err) => {
                error!("unable to write to config file: {:?}", err);
                std::process::exit(2)
            },
        };
    }
}

fn print_status(key: &str) {
    match env::var_os(key) {
        Some(val) => println!(" - {} is {} to: {:?}", key, style("currently set").green(), val),
        None => println!(" - {} is {}", key, style("not currently set").cyan()),
    }
}