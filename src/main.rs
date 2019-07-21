use ping_list::load_host;
use ping_list::Opt;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    println!("Hello, world!");
    // let opt = Opt::from_args();
    let path = PathBuf::from("server_list.json");
    match load_host(path) {
        Ok(_) => {}
        Err(e) => println!("error: {:?}", e),
    }
}
