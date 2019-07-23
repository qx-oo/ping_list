use crate::err::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::process;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

lazy_static! {
    static ref PING_REGEX: Regex = { Regex::new(r"\d{2} bytes from.*time=(?P<ts>.*) ms").unwrap() };
}

#[derive(Debug)]
pub struct HostInfo {
    pub host: String,
    pub time: f64,
    pub resolve: f64,
}

/// Ping hosts
///
/// # Usage:
/// ```
/// let test_lst: Vec<String> = vec!["www.google.com".to_owned()];
/// let info_lst = ping_host_list(&test_lst).unwrap();
/// ```
pub fn ping_host_list(hosts: &Vec<String>) -> Result<Vec<HostInfo>, Error> {
    let share_v = Arc::new(Mutex::new(Vec::<HostInfo>::new()));
    let child_lst = hosts
        .iter()
        .map(|host| {
            let local_v = share_v.clone();
            let h = host.clone();
            thread::spawn(move || {
                let start = Instant::now();
                let addr = match resolve(&h) {
                    Ok(n) => n,
                    Err(e) => {
                        println!("thread error: {:?}", e);
                        process::exit(1);
                    }
                };
                let duration = start.elapsed();
                let avg_ts = match ping_host(&addr) {
                    Ok(n) => n,
                    Err(e) => {
                        println!("thread error: {:?}", e);
                        process::exit(1);
                    }
                };
                let mut lst = local_v.lock().unwrap();
                lst.push(HostInfo {
                    host: h,
                    time: avg_ts,
                    resolve: duration.as_millis() as f64,
                });
                drop(lst);
            })
        })
        .collect::<Vec<_>>();
    for child in child_lst {
        child.join().unwrap();
    }
    let mut lst = share_v.lock().unwrap();
    lst.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    drop(lst);
    let lock = match Arc::try_unwrap(share_v) {
        Ok(res) => res,
        Err(_) => Err("Get share_v fail")?,
    };
    lock.into_inner().map_err(|_| "Get share_v fail".into())
}

/// Call systeam ping command.
pub fn ping_host(addr: &IpAddr) -> Result<f64, Error> {
    let ip = format!("{}", addr);
    Command::new("ping")
        .args(&["-c", "3", &ip])
        .output()
        .map(|ping| {
            let output = String::from_utf8_lossy(&ping.stdout).to_owned().to_string();
            let ts_lst = match_ping_ts(&output);
            if ts_lst.len() == 0 {
                std::f64::INFINITY
            } else {
                let sum = ts_lst.iter().sum::<f64>();
                let mut avg = sum / (ts_lst.len() as f64);
                avg = ((avg * 100f64) as i64) as f64;
                avg / 100f64
            }
        })
        .map_err(|e| e.into())
}

/// Get ip by host.
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

fn match_ping_ts(text: &String) -> Vec<f64> {
    PING_REGEX
        .captures_iter(&text)
        .map(|c| {
            let ts = match c.name("ts") {
                Some(n) => n.as_str(),
                None => "0",
            };
            match ts.parse::<f64>() {
                Ok(n) => n,
                Err(_) => 0f64,
            }
        })
        .filter(|&n| n != 0f64)
        .collect::<Vec<f64>>()
}

#[cfg(test)]
mod test {
    use super::{match_ping_ts, ping_host, ping_host_list, resolve};
    #[test]
    fn test_ping_host_list() {
        let test_lst: Vec<String> = vec!["www.google.com".to_owned(), "www.baidu.com".to_owned()];
        let info_lst = ping_host_list(&test_lst).unwrap();
        println!("info_lst: {:?}", info_lst);
    }
    #[test]
    fn test_resolve() {
        let ips = resolve("www.google.com");
        println!("ips: {:?}", ips);
        assert!(ips.is_ok());
        let ips = resolve("127.0.0.1");
        println!("ips: {:?}", ips);
        assert!(ips.is_ok());
    }
    #[test]
    fn test_ping_host() {
        let ipaddr = "223.6.6.6".parse().unwrap();
        let res = ping_host(&ipaddr);
        assert!(res.is_ok());
    }
    #[test]
    fn test_match_ping_ts() {
        let data = "PING 223.6.6.6 (223.6.6.6): 56 data bytes
        64 bytes from 223.6.6.6: icmp_seq=0 ttl=117 time=16.369 ms
        64 bytes from 223.6.6.6: icmp_seq=1 ttl=117 time=123d ms
        64 bytes from 223.6.6.6: icmp_seq=2 ttl=117 time=14.516 ms
        64 bytes from 223.6.6.6: icmp_seq=3 ttl=117 time=15.594 ms
        
        --- 223.6.6.6 ping statistics ---
        4 packets transmitted, 4 packets received, 0.0% packet loss
        round-trip min/avg/max/stddev = 14.516/15.291/16.369/0.745 ms"
            .to_owned();
        let lst = match_ping_ts(&data);
        println!("lst: {:?}", lst);
        assert_eq!(lst.len(), 3);
    }
}
