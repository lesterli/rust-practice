extern crate clap;
use clap::App;

pub fn example() {
    App::new("bitcoin")
        .version("1.0")
        .about("Bitcoin: Open Source P2P Money")
        .author("Bitcoin Core")
        .get_matches();
}