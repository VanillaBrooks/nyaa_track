use serde_bencode::{self, de};
use serde_derive::{Deserialize};
use serde_bytes::{self, ByteBuf};
use std::fs;
use std::io::prelude::*;


use super::super::error::*;


pub struct AnnounceResult {
    pub info_hash: String,
    pub announce_url: String,
    pub title: String,
    pub creation_date: i64,
    pub data: AnnounceData
}
impl AnnounceResult{
    pub fn new_bytes(input_bytes: &Vec<u8>, hash: String, url: String, title: String, date: i64) -> Result<AnnounceResult, Error>{
        let data = AnnounceData::new_bytes(&input_bytes)?;
        let s = AnnounceResult{info_hash: hash, announce_url: url, data: data, title: title, creation_date: date};
        Ok(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct AnnounceData  {
    pub complete: i64,
    pub incomplete: i64,
    pub downloaded: i64, 
    pub interval: i64,
    #[serde(default)]
    pub peers: Option<ByteBuf>,
    #[serde(default)]
    pub peers6: Option<ByteBuf>,
    // #[serde(default)]
    // info_hash: Option<&'a str>
}

impl AnnounceData {
    pub fn new_bytes(input_bytes: &Vec<u8>) ->Result<AnnounceData, Error> {
        let ann = de::from_bytes::<AnnounceData>(&input_bytes)?;
        Ok(ann)
    }
    // pub fn new_file(filename: &str) -> Result<AnnounceData, Error> {
    //     let mut buffer = Vec::new();
    //     let mut file = std::fs::File::open(filename)?;
    //     file.read_to_end(&mut buffer)?;
    //     AnnounceData::new_bytes(&buffer)
    // }
}