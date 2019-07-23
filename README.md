ping_list
===
Test Ping host list and help optimize network speed.

### Usage

Command:

    $ cargo build
    $ ping_list server_list.json

Library:
```
use ping_list::{load_host, ping_host, ping_host_list, HostInfo}

let test_lst: Vec<String> = vec!["www.google.com".to_owned()];
let info_lst = ping_host_list(&test_lst).unwrap();
println("Info List: {:?}", info_lst);
```