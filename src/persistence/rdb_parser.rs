use core::str;
use std::collections::HashMap;

use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

use crate::DbValue;
pub struct RdbParser {}

impl RdbParser {
    pub async fn parse(
        file_dir: String,
        db_filename: String,
    ) -> Result<HashMap<String, DbValue>, &'static str> {
        let file_path = &format!("{}/{}", file_dir, db_filename);
        let file = File::open(file_path).await;
        if file.is_err() {
            tracing::error!("File not found");
            return Err("File not found");
        }
        tracing::debug!("File path: {:?}", file_path);
        let mut buf_reader = BufReader::new(file.unwrap());
        let mut buffer: Vec<u8> = vec![];
        let _ = buf_reader.read_to_end(&mut buffer).await;
        tracing::trace!("Buffer: {:?}", buffer);
        let is_redis = buffer.starts_with(b"REDIS");
        if !is_redis {
            tracing::error!("Invalid persistence file");
            return Err("Invalid persistence file");
        }
        let map = HashMap::new();
        tracing::trace!("Searchinf for FB pos");
        let fb_pos = buffer.iter().position(|&b| b == 0xFB).unwrap();
        tracing::trace!("FB pos: {:?}", fb_pos);
        let db_size = RdbParser::get_db_size(&buffer, fb_pos);
        tracing::trace!("DB size: {:?}", db_size);
        return Ok(map);
    }

    fn get_db_size(buffer: &[u8], fb_pos: usize) -> u64 {
        let db_size_bytes = &buffer[fb_pos + 1..fb_pos + 9];
        tracing::trace!("DB size bytes: {:?}", db_size_bytes);
        tracing::trace!("DB size bytes in hex: {:02x?}", db_size_bytes);
        return u64::from_le_bytes(db_size_bytes.try_into().unwrap());
    }
}
