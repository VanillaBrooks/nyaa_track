use serde_bencode::{self, de};
use serde_derive::{Deserialize};
use serde_bytes::{self, ByteBuf};
use std::fs;
use std::io::prelude::*;


use super::super::error::*;


pub struct AnnounceResult<'a> {
    pub info_hash: &'a String,
    pub announce_url: &'a String,
    pub title: &'a String,
    pub creation_date: &'a i64,
    pub data: AnnounceData
}
impl <'a> AnnounceResult <'a> {
    pub fn new_bytes(input_bytes: &Vec<u8>, hash: &'a String, url: &'a String, title: &'a String, date: &'a i64) -> Result<AnnounceResult<'a>, Error>{
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