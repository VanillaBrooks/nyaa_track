use serde;
// use serde::Deserialize;

use serde_derive::{self, Serialize, Deserialize};

use serde_bytes::{self, ByteBuf};

use serde_bencode;
use serde_bencode::de;

use std::io::{self, Read};


#[derive(Debug, Deserialize)]
pub struct Node(String, i64);

#[derive(Debug, Deserialize)]
pub struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    name: String,
    // #[serde(default)]
    // pieces: ByteBuf,
    #[serde(rename="piece length")]
    piece_length: i64,
    #[serde(default)]
    md5sum: Option<String>,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)]
    files: Option<Vec<File>>,
    #[serde(default)]
    private: Option<u8>,
    #[serde(default)]
    path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="root hash")]
    root_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    #[serde(default)]
    info: Option<Info>,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename="creation date")]
    creation_date: Option<i64>,
    #[serde(rename="comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename="created by")]
    created_by: Option<String>,

    #[serde(default)]
    complete: Option<i64>,
    #[serde(default)]
    incomplete:Option<i64>,
    #[serde(default)]
    downloaded: Option<i64>,
}


impl Torrent{
    pub fn new_bytes(input_bytes: &Vec<u8>) ->Result<Torrent, serde_bencode::Error> {
        de::from_bytes::<Torrent>(&input_bytes)
    }
    pub fn new_file(filename: &str) -> Result<Torrent, serde_bencode::Error> {
        let mut buffer = Vec::new();
        let mut file = std::fs::File::open(filename).unwrap();
        file.read_to_end(&mut buffer);
        println!{"the buffer is:"}
        dbg!{&buffer};
        Torrent::new_bytes(&buffer)
    }
}
