use structopt::StructOpt;
use structopt::clap::Shell;

mod model;

fn main() {
    // generate bash completions
    model::LaunchArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

    // parse command line args (if passed in)
    let opt = model::LaunchArgs::from_args();

    // set verbosity
    let verbosity: u8 = opt.verbose;

    // parse config file
    let mut cfg: model::Config = match confy::load(model::APP_NAME) {
        Ok(data) => data,
        Err(err) => {
            println!("Error with config file: {:?}", err);
            std::process::exit(1);
        },
    };

    // parse cmd line option and execute
    match opt.cmd {
        Some(cmd) => match cmd {
            model::Command::Enable{} => cfg.enable_proxy(verbosity),
            model::Command::Disable{} => cfg.disable_proxy(verbosity),
            model::Command::List{} => cfg.list_proxies(verbosity),
            model::Command::Remove{ name } => cfg.remove_proxy(verbosity, name),
            model::Command::Add(proxy) => cfg.add_proxy(verbosity, proxy),
        },
        None => cfg.determine_proxy(verbosity)
    }
}