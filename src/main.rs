mod config;

use config::GlobalConf;

fn main() {
    println!("Hello, world!");
    GlobalConf::get_conf();
}
