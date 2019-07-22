use crate::err::Error;
use regex::Regex;
use std::collections::HashMap;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::process::Command;
use std::sync::{Arc, Mutex};
use lazy_static;

static PING_REGEX: &'static str = r"\d{2} bytes from.*time=(.*) ms";

lazy_static! {
    static ref PING_REGEX: &'static str = r"\d{2} bytes from.*time=(.*) ms";
    static ref reg: Regex = {
        match Regex::new(re: &str) {}
    }
}

pub fn ping_host_list(hosts: &Vec<String>) -> Result<HashMap<String, u32>, Error> {
    let share_v = Arc::new(Mutex::new(HashMap::<String, u32>::new()));
    let lock = match Arc::try_unwrap(share_v) {
        Ok(res) => res,
        Err(_) => Err("Get share_v share_v fail")?,
    };
    let val = match lock.into_inner() {
        Ok(res) => res,
        Err(_) => Err("Get share_v share_v fail")?,
    };
    Ok(val)
}

pub fn ping_host(addr: &IpAddr) -> Result<u32, Error> {
    let ip = format!("{}", addr);
    let ping = Command::new("ping").args(&["-c", "4", &ip]).output()?;
    let output = String::from_utf8_lossy(&ping.stdout);
    println!("output: {:?}", output);
    Ok(0)
}

fn resolve(host: &str) -> Result<IpAddr, Error> {
    let ips: std::io::Result<Vec<IpAddr>> = (host, 0)
        .to_socket_addrs()
        .map(|iter| iter.map(|socket_address| socket_address.ip()).collect());
    match ips {
        Ok(n) => {
            if n.len() > 0 {
                Ok(n[0])
            } else {
                Err("resolve host fail")?
            }
        }
        Err(_) => Err("resolve host fail")?,
    }
}

fn match_ping_ts(text: &String) -> Option<f64> {
    let re = Regex::new(PING_REGEX);
}

// fn send_icmp_packet(addr: IpAddr, timeout: u64) -> Result<u32, Error> {
//     let icmp = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
//     Ok(2)
// }

#[cfg(test)]
mod test {
    use super::{ping_host, resolve};
    // use super::send_icmp_packet;
    #[test]
    fn test_resolve() {
        let ips = resolve("www.google.com");
        println!("ips: {:?}", ips);
        assert!(ips.is_ok());
    }
    #[test]
    fn test_ping_host() {
        let ipaddr = "223.6.6.6".parse().unwrap();
        match ping_host(&ipaddr) {
            Ok(res) => println!("res: {:?}", res),
            Err(e) => println!("error: {:?}", e),
        }
    }
    // #[test]
    // fn test_send_icmp_packet() {
    //     let ipaddr = "223.6.6.6".parse().unwrap();
    //     match send_icmp_packet(ipaddr, 1) {
    //         Ok(t) => println!("t: {}", t),
    //         Err(e) => {
    //             println!("error: {:?}", e);
    //         }
    //     };
    //     assert!(true)
    // }
}
