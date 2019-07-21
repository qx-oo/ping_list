use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic", about = "Usage")]
pub struct Opt {
    #[structopt(help = "json config file")]
    pub config: PathBuf,
}
