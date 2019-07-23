//! This is call system ping command and parse time
//!
//! # Usage:
//! ```
//! use ping_list::{load_host, ping_host, ping_host_list, HostInfo}
//! ```
mod err;
mod load_host;
mod opt;
mod ping;

pub use err::Error;
pub use load_host::load_host;
pub use opt::Opt;
pub use ping::{ping_host, ping_host_list, HostInfo};
