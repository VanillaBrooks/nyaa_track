use serde;
// use serde::Deserialize;

use serde_derive::{self, Serialize, Deserialize};

use serde_bytes::{self, ByteBuf};

use serde_bencode;
use serde_bencode::de;

use std::io::{self, Read};

use crypto;
use crypto::digest::Digest;

use bencode::ToBencode;
use bencode::Bencode;
use std::collections::BTreeMap;
use bencode::util::ByteString;

use std::fs;
use std::path::Path;



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
    #[serde(rename = "length")]
    // #[serde(default)]
    length: u64,
    name: String,
    
    #[serde(rename="piece length")]
    piece_length: u64,
    
    #[serde(default)]
    pieces: ByteBuf,
    
    #[serde(default)]
    private: u8,

    #[serde(default)]
    md5sum: Option<String>,
    
    #[serde(default)]
    files: Option<Vec<File>>,
    
    
    #[serde(default)]
    path: Option<Vec<String>>,
    
    #[serde(default)]
    #[serde(rename="root hash")]
    
    root_hash: Option<String>,
    #[serde(default)]
    info_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub info: Info,
    #[serde(default)]
    pub announce: Option<String>,
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
    pub creation_date: Option<u64>,
    #[serde(rename="comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename="created by")]
    created_by: Option<String>,
}

//TODO: Fix use lifetimes and return reference to hash instead
impl Torrent{
    pub fn new_bytes(input_bytes: &Vec<u8>) ->Result<Torrent, serde_bencode::Error> {
        de::from_bytes::<Torrent>(&input_bytes)
    }
    pub fn new_file(filename: &str) -> Result<Torrent, serde_bencode::Error> {
        let mut buffer = Vec::new();
        let mut file = std::fs::File::open(filename).unwrap();
        file.read_to_end(&mut buffer);
        Torrent::new_bytes(&buffer)
    }
    pub fn info_hash(&mut self) -> Result<String, std::io::Error> {
    match &self.info.info_hash{
        
        Some(x) => return Ok(x.to_string()),

        None => {
            let mut hasher = crypto::sha1::Sha1::new();
            let bencoded = self.to_bencode();
            // dbg!{&bencoded};
            let bytes = bencoded.to_bytes()?;


            hasher.input(&bytes);
            Ok(hasher.result_str())
        }
    }
    }
}

impl ToBencode for Torrent {
    fn to_bencode(&self) -> Bencode {
        let mut m = BTreeMap::new();
        m.insert(ByteString::from_str("length"), self.info.length.to_bencode());
        m.insert(ByteString::from_str("name"), self.info.name.to_bencode());
        m.insert(ByteString::from_str("piece length"), self.info.piece_length.to_bencode());
        m.insert(ByteString::from_str("pieces"), Bencode::ByteString(self.info.pieces.clone().into_vec()));
        // m.insert(ByteString::from_str("private"), self.info.private.to_bencode());
        // dbg!{&m};
        Bencode::Dict(m)
    }
}

#[derive(Debug, Deserialize)]
pub struct Announce {
    pub complete: u32,
    pub incomplete: u32,
    pub downloaded: u32, 
    pub interval: u64,
    #[serde(default)]
    pub peers: Option<ByteBuf>,
    #[serde(default)]
    pub peers6: Option<ByteBuf>
}

impl Announce {
    pub fn new_bytes(input_bytes: &Vec<u8>) ->Result<Announce, serde_bencode::Error> {
        de::from_bytes::<Announce>(&input_bytes)
    }
    pub fn new_file(filename: &str) -> Result<Announce, serde_bencode::Error> {
        let mut buffer = Vec::new();
        let mut file = std::fs::File::open(filename).unwrap();
        file.read_to_end(&mut buffer);
        Announce::new_bytes(&buffer)
    }
}


#[derive(Debug, Deserialize)]
pub struct TestInfo {
    length : u64,
    name: String,
    #[serde(rename= "piece length")]
    piece_length: u64
}

impl TestInfo{
    pub fn new(filename : &str) -> Result<TestInfo, serde_bencode::Error> {
        let mut buffer = Vec::new();
        let mut file = std::fs::File::open(filename).unwrap();
        file.read_to_end(&mut buffer);
        de::from_bytes::<TestInfo>(&buffer)
    }
}