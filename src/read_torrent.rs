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

#[derive(Debug, Deserialize, Clone)]
pub struct File {
    pub length: u64,
    path: Vec<String>,
    #[serde(default)]
    md5sum: Option<String>,
}

impl ToBencode for File {
    fn to_bencode(&self) -> Bencode{
        let mut m = BTreeMap::new();
        // println!{"length"}
        m.insert(ByteString::from_str("length"), self.length.to_bencode());
        // println!{"path"}
        m.insert(ByteString::from_str("path"), self.path.to_bencode());

        if self.md5sum.is_some(){
            // println!{":::md5"}
            m.insert(ByteString::from_str("md5sum"), self.md5sum.clone().unwrap().to_bencode());
        }
        Bencode::Dict(m)
    }
}

#[derive(Debug, Deserialize)]
pub struct Info {
    #[serde(rename = "length")]
    #[serde(default)]
    length: Option<u64>,
    
    #[serde(default)]
    name: Option<String>,
    
    #[serde(rename="piece length")]
    piece_length: Option<u64>,
    
    #[serde(default)]
    pieces: Option<ByteBuf>,
    
    #[serde(default)]
    private: Option<u8>,

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

impl Info {
    pub fn new_bytes(bytes: &Vec<u8>) -> Result<Info, serde_bencode::Error> {
        de::from_bytes::<Info>(&bytes)
    }
}

impl ToBencode for Info {
    fn to_bencode(&self) -> Bencode {
        println!{"\n\n\n\nbencoding\n\n\n\n"}
        
        let mut m = BTreeMap::new();

        if self.length.is_some(){
            println!{"length"}
            m.insert(ByteString::from_str("length"), self.length.unwrap().to_bencode());
        }
        else if self.files.is_some() {
            println!{"files"}
            let bc = self.files.clone().unwrap().to_bencode();
            m.insert(ByteString::from_str("files"), bc);
        }

        match &self.name{
            Some(name) => {
                println!{"name"};
                m.insert(ByteString::from_str("name"), name.to_bencode());
            },
            None => {}
        }
        if self.piece_length.is_some() {
            println!{"piece length"}
            m.insert(ByteString::from_str("piece length"), self.piece_length.unwrap().to_bencode());
        }

        if self.pieces.is_some() {
            println!{"pieces"}
            m.insert(ByteString::from_str("pieces"), Bencode::ByteString(self.pieces.clone().unwrap().into_vec()));
        }
        if self.private.is_some() {
            println!{"private"}
            m.insert(ByteString::from_str("private"), self.private.unwrap().to_bencode());
        }

        Bencode::Dict(m)
    }
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
                let bytes = bencoded.to_bytes()?;


                hasher.input(&bytes);
                Ok(hasher.result_str())
            }
        }
    }
}

impl ToBencode for Torrent {
    fn to_bencode(&self) -> Bencode{
        Info::to_bencode(&self.info)
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
    length : Option<u64>,
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

pub fn sha1(bytes: &Vec<u8>) -> String {
        let mut hasher = crypto::sha1::Sha1::new();
        hasher.input(&bytes);
        return hasher.result_str();
}