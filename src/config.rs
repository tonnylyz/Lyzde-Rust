use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Config {
    conn : String,
    port : u16,
}

lazy_static! {
    static ref CONFIG: Config = {
        let mut config = Config {
            conn : String::from(""),
            port : 8000,
        };
        let mut file = BufReader::new(File::open("config.txt").unwrap());
        loop {
            let mut line = String::new();
            match file.read_line(&mut line) {
                Err(_) => { break; }
                Ok(0) => { break; } // met EOF
                Ok(_) => {
                    match extract_kv(&line) {
                        None => {}
                        Some((k, v)) => {
                            match k.as_str() {
                                "conn" => {
                                    config.conn = v.clone();
                                }
                                "port" => {
                                    match v.parse::<u16>() {
                                        Ok(port) => {
                                            config.port = port;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        config
    };
}

pub fn conn() -> String {
    (*CONFIG).conn.clone()
}

pub fn port() -> u16 {
    (*CONFIG).port
}

fn extract_kv(line : &String) -> Option<(String, String)> {
    let list = line.split('=').collect::<Vec<_>>();
    if list.len() != 2 {
        None
    } else {
        Some((list[0].trim().to_string(), list[1].trim().to_string()))
    }
}
