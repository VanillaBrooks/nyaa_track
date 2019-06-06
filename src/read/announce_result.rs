use serde_bencode::{self, de};
use serde_derive::{Deserialize};
use serde_bytes::{self, ByteBuf};

use super::super::error::*;
use super::super::utils;


pub struct AnnounceResult {
    pub info_hash: String,
    pub announce_url: String,
    pub title: String,
    pub poll_time: i64,
    pub creation_date: i64,
    pub data: AnnounceData
}
impl <'a> AnnounceResult <'a> {
    pub fn new_bytes(input_bytes: &Vec<u8>, hash: &'a String, url: &'a String, title: &'a String, date: &'a i64) -> Result<AnnounceResult<'a>, Error>{
        let data = AnnounceData::new_bytes(&input_bytes)?;
        let s = AnnounceResult{info_hash: hash, 
        announce_url: url,
        data: data,
        title: title, 
        creation_date: date,
        poll_time: utils::get_unix_time()};
        Ok(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct AnnounceData  {
    pub complete: i64, //seeds
    pub incomplete: i64, // downloading now
    pub downloaded: i64,  // snatches
    pub interval: i64,
    #[serde(default)]
    pub peers: Option<ByteBuf>,
    #[serde(default)]
    pub peers6: Option<ByteBuf>,
}

impl AnnounceData {
    pub fn new_bytes(input_bytes: &Vec<u8>) ->Result<AnnounceData, Error> {
        let ann = de::from_bytes::<AnnounceData>(&input_bytes)?;
        Ok(ann)
    }
}