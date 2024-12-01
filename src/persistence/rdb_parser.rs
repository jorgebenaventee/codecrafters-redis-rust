use core::str;
use std::collections::HashMap;

use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

use crate::{DbValue, DB};
pub struct RdbParser {}

impl RdbParser {
    pub async fn parse(
        file_dir: String,
        db_filename: String,
    ) -> Result<HashMap<String, DbValue>, &'static str> {
        let file_path = &format!("{}/{}", file_dir, db_filename);
        let file = File::open(file_path).await;
        if file.is_err() {
            return Err("File not found");
        }
        println!("File path: {:?}", file_path);
        let mut buf_reader = BufReader::new(file.unwrap());
        let mut buffer: Vec<u8> = vec![];
        let _ = buf_reader.read_to_end(&mut buffer).await;
        let is_redis = buffer.starts_with(b"REDIS");
        if !is_redis {
            return Err("Invalid persistence file");
        }
        let fb_pos = buffer.iter().position(|&b| b == 0xFB).unwrap();
        let mut pos = fb_pos + 4;
        let len = buffer[pos];
        pos += 1;
        let key = &buffer[pos..(pos + len as usize)];
        println!("Key bytes: {:?}", key);
        let key = str::from_utf8(key).unwrap();
        let mut map: HashMap<String, DbValue> = HashMap::new();
        map.insert(
            key.to_string(),
            DbValue {
                value: "hello".to_string(),
                expires_at: None,
            },
        );
        return Ok(map);
    }
}
