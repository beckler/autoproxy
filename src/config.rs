extern crate confy;
extern crate serde_derive;

use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    enabled: Option<bool>,
    proxy: Option<Vec<Proxy>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    http: Option<String>,
    https: Option<String>,
    no: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enabled: Some(false),
            proxy: Some(vec![Proxy {
                http: None,
                https: None,
                no: None,
            }])
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// pub fn parse_config(config_path: Option<PathBuf>) -> Config {
//     let config_file_path: PathBuf = match config_path {
//         Some(buf) => buf,
//         None
//         // None => get_default_config(),
//     };

//     let contents: String = match fs::read_to_string(config_file_path) {
//         Ok(data) => data,
//         Err(err) => {
//             println!("{:?}", err);
//             println!("Unable to find the autoproxy config in the expected path");
//             process::exit(1);
//         }
//     };

//     return match toml::from_str(&contents) {
//         Ok(data) => data,
//         Err(err) => {
//             println!("Unable to parse autoproxy config! Reason: {:?}", err);
//             process::exit(1)
//         }
//     };
// }
