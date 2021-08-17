use std::path::{PathBuf};
use dirs::config_dir;
use structopt::StructOpt;
use nix::unistd::getuid;
use confy::{load_path, store_path};
use serde::{Serialize, Deserialize};

#[derive(StructOpt, Debug)]
#[structopt(name = "clashup", about = "A clash manager")]
struct ArgConf {
    #[structopt(short, help = "config file path")]
    config_path: Option<PathBuf>,
}

pub struct GlobalConf {
    config_path: PathBuf,
    is_root: bool,
}

impl GlobalConf {
     pub fn get_conf() -> Self {
        let conf = ArgConf::from_args();
        let is_root = getuid().is_root();
        let path = match conf.config_path {
            None => match is_root {
                true => PathBuf::from("/etc/clash/clashup.toml"),
                false => match config_dir() {
                    Some(mut user_conf_dir) => {
                        user_conf_dir.push("clash/clashup.toml");
                        user_conf_dir
                    },
                    None => panic!("Unable to get user config dir")
                }
            },
            Some(path) => path,
        };
        GlobalConf {
            config_path: path,
            is_root: is_root
        }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    clash_path: ClashPath,
    update_url: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ClashPath {
    System,
    Local,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self { clash_path: ClashPath::Local, update_url: "fake url".into() }
    }
}


#[cfg(test)]
mod test {
    use tempfile::TempDir;
    use super::*;
    #[test]
    fn config_dump() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_config_file = tmp_dir.path().join("clashup.toml");
        let dummy_conf = Config { clash_path: ClashPath::Local, update_url: "dummp".into() };
        store_path(&tmp_config_file, &dummy_conf).expect("error store");
        let read_conf: Config = load_path(&tmp_config_file).expect("error load");
        assert_eq!(dummy_conf, read_conf);
    }
}
