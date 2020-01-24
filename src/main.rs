use structopt::StructOpt;
use structopt::clap::Shell;

mod model;

use std::net::TcpStream;

static mut VERBOSITY: u8 = 0;

fn main() {
    // generate bash completions
    model::LaunchArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

    // parse command line args (if passed in)
    let opt = model::LaunchArgs::from_args();

    // set verbosity
    unsafe {
        VERBOSITY = opt.verbose;
    }

    // parse config file
    let cfg: model::Config = match confy::load("autoproxy") {
        Ok(data) => data,
        Err(err) => {
            println!("Error with config file: {:?}", err);
            std::process::exit(1);
        },
    };

    println!("{:?}", cfg);

    if let Ok(_) = TcpStream::connect("google.com:443") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }

    // parse cmd line option and execute
    match opt.cmd {
        Some(cmd) => match cmd {
            model::Command::Enable{} => enable_proxy(),
            model::Command::Disable{} => disable_proxy(),
            model::Command::List{ json } => list_proxies(json),
            model::Command::Remove{ name } => remove_proxy(name),
            model::Command::Add(proxy_type) => add_proxy(proxy_type),
        },
        None => std::process::exit(0)
    }
}

fn enable_proxy() {
    std::process::exit(0);
}

fn disable_proxy() {
    std::process::exit(0);
}

fn list_proxies(json: bool) {
    println!("BOOOOOL: {:?}", json);
    std::process::exit(0);
}

fn remove_proxy(name: String) {
    println!("stringggg: {:?}", name);
    std::process::exit(0);
}

fn add_proxy(proxy_type: model::ProxyType) {
    println!("proxy bby: {:?}", proxy_type);
    std::process::exit(0);
}