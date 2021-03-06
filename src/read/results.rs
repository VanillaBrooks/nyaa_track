use serde_bencode::{self, de};
use serde_bytes::{self, ByteBuf};
use serde_derive::Deserialize;

use super::super::error::*;
use super::super::utils;

use std::collections::HashMap;

use std::sync::Arc;

#[derive(Debug)]
pub struct AnnounceResult<'a> {
    pub info_hash: &'a str,
    pub announce_url: &'a str,
    pub title: &'a str,
    pub poll_time: i64,
    pub creation_date: i64,
    pub data: AnnounceData,
}
impl<'a> AnnounceResult<'a> {
    pub fn new_bytes(
        input_bytes: &[u8],
        hash: &'a str,
        url: &'a str,
        title: &'a str,
        date: i64,
    ) -> Result<AnnounceResult<'a>, Error> {
        let data = AnnounceData::new_bytes(&input_bytes)?;
        let s = AnnounceResult {
            info_hash: hash,
            announce_url: url,
            data,
            title,
            creation_date: date,
            poll_time: utils::get_unix_time(),
        };
        Ok(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct AnnounceData {
    pub complete: i64,   //seeds
    pub incomplete: i64, // downloading now
    pub downloaded: i64, // snatches
    #[serde(default)]
    pub interval: Option<i64>,
    #[serde(default)]
    pub peers: Option<ByteBuf>,
    #[serde(default)]
    pub peers6: Option<ByteBuf>,
}

impl AnnounceData {
    pub fn new_bytes(input_bytes: &[u8]) -> Result<AnnounceData, Error> {
        let ann = de::from_bytes::<AnnounceData>(&input_bytes)?;
        Ok(ann)
    }
}

pub struct ScrapeResult<'a> {
    pub info_hash: &'a str,
    pub announce_url: &'a str,
    pub title: &'a str,
    pub poll_time: i64,
    pub creation_date: i64,
    pub data: ScrapeData,
}

impl<'a> ScrapeResult<'a> {
    pub fn new_bytes(
        input_bytes: &[u8],
        hash: &'a str,
        url: &'a str,
        title: &'a str,
        date: i64,
    ) -> Result<ScrapeResult<'a>, Error> {
        let data = ScrapeData::new_bytes(&input_bytes)?;
        let s = ScrapeResult {
            info_hash: hash,
            announce_url: url,
            data,
            title,
            creation_date: date,
            poll_time: utils::get_unix_time(),
        };
        Ok(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct ScrapeData {
    pub files: HashMap<ByteBuf, File>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub complete: i64,   //seeds
    pub incomplete: i64, // downloading now
    pub downloaded: i64, // snatches
}

impl ScrapeData {
    pub fn new_bytes(input_bytes: &[u8]) -> Result<ScrapeData, Error> {
        let ann = de::from_bytes::<ScrapeData>(&input_bytes)?;
        Ok(ann)
    }
}

#[derive(Debug, Clone)]
pub struct GenericData {
    pub hash: Arc<String>,
    pub url: Arc<String>,
    pub creation_date: i64,
    pub title: Arc<String>,
    pub downloaded: i64,
    pub complete: i64,
    pub incomplete: i64,
    pub poll_time: i64,
}

// impl <'a> GenericData <'a> {
impl GenericData {
    pub fn new(
        hash: Arc<String>,
        url: Arc<String>,
        date: i64,
        title: Arc<String>,
        downloaded: i64,
        complete: i64,
        incomplete: i64,
    ) -> GenericData {
        GenericData {
            hash,
            url,
            creation_date: date,
            title,
            downloaded,
            complete,
            incomplete,
            poll_time: utils::get_unix_time(),
        }
    }
}
