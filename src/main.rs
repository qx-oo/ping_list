use ping_list::{load_host, ping_host_list, Error, Opt};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    println!("Start ...");
    let opt = Opt::from_args();
    let path = PathBuf::from(opt.config);
    let hosts = load_host(path)?;
    let info_lst = ping_host_list(&hosts)?;
    println!();
    println!(
        "{:<25}|  {:<15}|  {:<15}",
        "Host List", "Ping Time(ms)", "DNS Time(ms)"
    );
    println!("{:-<25}|{:-<17}|{:-<17}", "", "", "");
    for info in info_lst {
        let ts = if std::f64::INFINITY == info.time {
            "timeout".to_owned()
        } else {
            info.time.to_string()
        };
        println!("{:<25}|  {:<15}|  {:<15}", info.host, ts, info.resolve);
    }
    Ok(())
}
