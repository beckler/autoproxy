use log::{debug, error};
use structopt::StructOpt;
use structopt::clap::Shell;
use structopt::clap::AppSettings;

mod model;

// cli structs
#[derive(StructOpt, Debug)]
#[structopt(about, global_settings = &[AppSettings::DisableVersion])]
struct LaunchArgs{
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,
    // subcommand
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Prints the current status
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
    Add(model::Proxy),
    /// Removes an existing proxy configuration
    #[structopt(name = "remove")]
    Remove {
        /// Proxy configuration alias
        name: String,
    },
}

fn main() {
    // generate bash completions
    LaunchArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

    // parse command line args (if passed in)
    let opt = LaunchArgs::from_args();

    // setup logging
    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .unwrap();

    // parse config file
    debug!("retrieving and parsing config file");
    let mut cfg: model::Config = match confy::load(model::APP_NAME) {
        Ok(data) => data,
        Err(err) => {
            error!("Unable to load config file: {:?}", err);
            std::process::exit(1);
        },
    };
    debug!("successfully loaded config file");

    // parse cmd line option and execute
    match opt.cmd {
        Some(cmd) => match cmd {
            Command::Status{} => cfg.status(),
            Command::Enable{} => cfg.enable_proxy(),
            Command::Disable{} => cfg.disable_proxy(),
            Command::List{} => cfg.list_proxies(),
            Command::Remove{ name } => cfg.remove_proxy(name),
            Command::Add(proxy) => cfg.add_proxy(proxy),
        },
        None => cfg.determine_proxy()
    }
}