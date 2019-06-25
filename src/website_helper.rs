pub fn fetch_kv(k : &str, default: Option<&str>) -> String {
    match crate::db_helper::query_single(format!("SELECT `v` FROM `kv` WHERE `k`='{}'", k).as_str()) {
        Ok(s) => { s }
        Err(_) => { match default {
            None => { "".to_string() },
            Some(d) => { d.to_string() },
        }}
    }
}
