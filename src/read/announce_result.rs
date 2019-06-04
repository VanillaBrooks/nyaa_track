use serde_bencode::{self, de};
use serde_derive::{Deserialize};
use serde_bytes::{self, ByteBuf};
use std::fs;
use std::io::prelude::*;


use super::super::error::*;


#[derive(Debug, Deserialize)]
pub struct AnnounceResult {
    pub complete: i64,
    pub incomplete: i64,
    pub downloaded: i64, 
    pub interval: i64,
    #[serde(default)]
    pub peers: Option<ByteBuf>,
    #[serde(default)]
    pub peers6: Option<ByteBuf>
}

impl AnnounceResult {
    pub fn new_bytes(input_bytes: &Vec<u8>) ->Result<AnnounceResult, Error> {
        let ann = de::from_bytes::<AnnounceResult>(&input_bytes)?;
        Ok(ann)
    }
    pub fn new_file(filename: &str) -> Result<AnnounceResult, Error> {
        let mut buffer = Vec::new();
        let mut file = std::fs::File::open(filename)?;
        file.read_to_end(&mut buffer)?;
        AnnounceResult::new_bytes(&buffer)
    }
}