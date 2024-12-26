use core::str;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

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
        let buffer = RdbParser::read_file(file_path.to_string()).await;
        let mut map = HashMap::new();
        if buffer.is_empty() {
            return Ok(map);
        }
        let fb_pos = RdbParser::get_fb_pos(&buffer);
        let db_size = RdbParser::get_db_size(&buffer, fb_pos);
        tracing::trace!("DB size: {:?}", db_size);
        let mut value_type_pos = fb_pos + 4;
        for _ in 0..db_size {
            println!("\r\n\r\n");
            tracing::trace!("Value type pos: {:?}", value_type_pos);
            let key_length_pos = if buffer[value_type_pos - 1] == 0xFC {
                value_type_pos + 9
            } else {
                value_type_pos
            };
            let type_byte = buffer[value_type_pos];
            tracing::trace!("Value type: {:?}", type_byte);
            let key_length = buffer[key_length_pos];
            let key = RdbParser::get_key(&buffer, key_length_pos, key_length);
            tracing::trace!("Key: {:?}", key);
            let value_length_pos = key_length_pos + key_length as usize + 1;
            let value_length = buffer[value_length_pos];
            let value = RdbParser::get_value(&buffer, value_length_pos, value_length);
            tracing::trace!("Value: {:?}", value);
            tracing::debug!(
                "Buffer at value_type_pos - 1: 0x{:02x?}",
                &buffer[value_type_pos - 1]
            );
            let expires_at: Option<u128> = if buffer[value_type_pos - 1] == 0xFC {
                let expires_at_vec: Vec<u8> = buffer[value_type_pos..=(value_type_pos + 7)]
                    .iter()
                    .map(|&b| b)
                    .collect();
                let mut expires_at: u128 = 0;
                for (i, &byte) in expires_at_vec.iter().enumerate() {
                    expires_at |= (byte as u128) << (i * 8);
                }
                tracing::trace!("Expires at vec: {:?}", expires_at_vec);
                tracing::trace!("Expires at: {:?}", expires_at);
                Some(expires_at as u128)
            } else {
                None
            };
            map.insert(key, DbValue { value, expires_at });
            value_type_pos = value_length_pos + value_length as usize + 2;
        }
        Ok(map)
    }

    fn get_db_size(buffer: &[u8], fb_pos: usize) -> u8 {
        buffer[fb_pos + 1]
    }

    fn get_key(buffer: &[u8], key_length_pos: usize, key_length: u8) -> String {
        tracing::trace!("Key length: {:?}", key_length as usize);
        tracing::trace!("Key length pos: {:?}", key_length_pos);
        tracing::trace!(
            "Seaching key from pos {:?} to pos {:?}",
            key_length_pos + 1,
            key_length_pos + key_length as usize
        );
        let key_bytes = &buffer[key_length_pos + 1..=(key_length_pos + key_length as usize)];
        let key = str::from_utf8(key_bytes).unwrap();
        key.to_string()
    }

    fn get_value(buffer: &[u8], value_length_pos: usize, value_length: u8) -> String {
        tracing::trace!("Value length: {:?}", value_length);
        tracing::trace!("Value length pos: {:?}", value_length_pos);
        tracing::trace!(
            "Seaching value from pos {:?} to pos {:?}",
            value_length_pos + 1,
            value_length_pos + value_length as usize
        );
        let value_bytes =
            &buffer[value_length_pos + 1..(value_length_pos + value_length as usize + 1)];
        let value = str::from_utf8(value_bytes).unwrap();
        value.to_string()
    }

    async fn read_file(file_path: String) -> Vec<u8> {
        let file = File::open(file_path).await;
        if file.is_err() {
            tracing::error!("File not found");
            return Vec::new();
        }
        let mut buf_reader = BufReader::new(file.unwrap());
        let mut buffer: Vec<u8> = vec![];
        let _ = buf_reader.read_to_end(&mut buffer).await;
        let is_redis = buffer.starts_with(b"REDIS");
        if !is_redis {
            tracing::error!("Invalid persistence file");
            return Vec::new();
        }
        buffer
    }

    fn get_fb_pos(buffer: &[u8]) -> usize {
        tracing::trace!("Searching for FB pos");
        let fb_pos = buffer.iter().position(|&b| b == 0xFB).unwrap();
        tracing::trace!("FB pos: {:?}", fb_pos);
        fb_pos
    }
}
