use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Config {
    conn: String,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            conn: "".to_string(),
            port: 8000,
        }
    }
}

fn read_config() -> Config {
    let mut config = Config::default();
    let r = File::open("config.txt");
    if let Err(_) = r {
        panic!("Open config file failed");
    }
    let r = r.unwrap();
    let mut file = BufReader::new(r);
    loop {
        let mut line = String::new();
        match file.read_line(&mut line) {
            Err(_) => { break; }
            Ok(0) => { break; } // met EOF
            Ok(_) => {
                if let Some((k, v)) = extract_kv(&line) {
                    match k.as_str() {
                        "conn" => {
                            config.conn = v.clone();
                        }
                        "port" => {
                            if let Ok(port) = v.parse::<u16>() {
                                config.port = port;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    config
}

lazy_static! {
    static ref CONFIG: Config = read_config();
}

pub fn conn() -> String {
    (*CONFIG).conn.clone()
}

pub fn port() -> u16 {
    (*CONFIG).port
}

fn extract_kv(line: &String) -> Option<(String, String)> {
    let list = line.split('=').collect::<Vec<_>>();
    if list.len() == 2 {
        Some((list[0].trim().to_string(), list[1].trim().to_string()))
    } else {
        None
    }
}
