use crate::err::Error;
use tokio_ping::{Pinger};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::IpAddr;

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

pub fn ping_host(host: &String) {
    // let ipaddr = IpAddr::from(host);
    // let pinger = Pinger::new();
    // let stream = pinger.and_then(move |pingger| {
    //     Ok(pinger.chain(addr).stream());
    // });
}
