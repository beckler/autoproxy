use std::{
    fs,
    // env,
    process,
    path::PathBuf,
};
use directories::{
    BaseDirs,
    ProjectDirs,
};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    enabled: Option<bool>,
    proxy: Option<Vec<Proxy>>,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    http: Option<String>,
    https: Option<String>,
    no: Option<String>,
}

fn get_default_config() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        let config_dir = base_dirs.home_dir().join(".config/autoproxy");
        if config_dir.exists() {
            return config_dir.to_path_buf()
        }
    }
    ProjectDirs::from("org", "beckler", "autoproxy")
        .expect("Unable to find configuration directories")
        .config_dir()
        .to_path_buf()
}

pub fn parse_config(config_path: Option<PathBuf>) -> Config {
    let config_file: PathBuf = match config_path {
        Some(buf) => buf,
        None => get_default_config(),
    };

    let contents: String = match fs::read_to_string(config_file) {
        Ok(data) => data,
        Err(_) => {
            println!("Unable to parse autoproxy config");
            process::exit(1);
        }
    };

    toml::from_str(&contents).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}