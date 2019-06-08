use std::fs::File;
use std::io::{BufRead, BufReader};

struct Config {
    conn : String,
    port : u16,
}

lazy_static! {
    static ref CONFIG: Config = {
        let mut conn = String::new();
        let mut port = String::new();
        let mut file = BufReader::new(File::open("config.txt").unwrap());

        file.read_line(&mut conn).unwrap();
        file.read_line(&mut port).unwrap();
        Config {
            conn : conn.trim_end().to_string(),
            port : port.parse::<u16>().unwrap(),
        }
    };
}

pub fn conn() -> String {
    (*CONFIG).conn.clone()
}

pub fn port() -> u16 {
    (*CONFIG).port
}
