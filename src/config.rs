use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    pub static ref CONN_STR: String = {
        let mut buf = String::new();
        let f = File::open("config.txt").unwrap();
        BufReader::new(f).read_line(&mut buf).unwrap();
        println!("conn_str: {}", buf);
        buf
    };
}
