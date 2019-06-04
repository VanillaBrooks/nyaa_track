use serde;

use serde_derive::{self, Serialize, Deserialize};

use serde_bytes::{self, ByteBuf};

use serde_bencode;
use serde_bencode::de;
// use serde_bencode::ser;

use std::io::{self, Read};

use crypto;
use crypto::digest::Digest;

use bencode::ToBencode;
use bencode::Bencode;
use std::collections::BTreeMap;
use bencode::util::ByteString;

use std::fs;
use std::path::Path;

use super::error::*;

use super::utils;
use super::requests::url_encoding;
// use regex::bytes::Regex;



// #[derive(Debug, Deserialize)]
// pub struct Node(String, i64);

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct File {
    #[serde(default)]
    ed2k: Option<ByteBuf>,

    #[serde(default)]
    filehash: Option<ByteBuf>,

    #[serde(default)]
    attr:Option<String>,

    pub length: i64,

    path: Vec<String>,

    #[serde(rename="path.utf-8")]
    #[serde(default)]
    utf8path: Option<Vec<String>>,

    #[serde(default)]
    md5sum: Option<String>,
}

impl ToBencode for File {
    fn to_bencode(&self) -> Bencode{
        let mut m = BTreeMap::new();

        if self.ed2k.is_some() {
            m.insert(ByteString::from_str("ed2k"), Bencode::ByteString(self.ed2k.clone().unwrap().to_vec()));
        }

        if self.filehash.is_some() {
            m.insert(ByteString::from_str("filehash"), Bencode::ByteString(self.filehash.clone().unwrap().to_vec()));
        }

        if self.attr.is_some(){
            m.insert(ByteString::from_str("attr"), self.attr.clone().unwrap().to_bencode());
        }

        m.insert(ByteString::from_str("length"), self.length.to_bencode());

        m.insert(ByteString::from_str("path"), self.path.to_bencode());

        if self.utf8path.is_some() {
            m.insert(ByteString::from_str("path.utf-8"), self.utf8path.clone().unwrap().to_bencode());
        }

        if self.md5sum.is_some(){
            m.insert(ByteString::from_str("md5sum"), self.md5sum.clone().unwrap().to_bencode());
        }

        Bencode::Dict(m)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    #[serde(rename="file-duration")]
    #[serde(default)]
    fileduration: Option<Vec<i64>>,

    #[serde(rename="file-media")]
    #[serde(default)]
    filemedia : Option<Vec<i64>>,

    #[serde(default)]
    ed2k: Option<ByteBuf>,

    #[serde(default)]
    filehash: Option<ByteBuf>,

    #[serde(rename = "length")]
    #[serde(default)]
    length: Option<i64>,
    
    #[serde(default)]
    name: Option<String>,

    #[serde(rename="name.utf-8")]
    #[serde(default)]
    utf8name: Option<String>,
    
    #[serde(rename="piece length")]
    piece_length: Option<i64>,
    
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
    source: Option<String>,
    
    #[serde(default)]
    #[serde(rename="root hash")]
    roothash: Option<String>,
    
    #[serde(default)]
    info_hash: Option<String>,
}

impl Info {
    pub fn new_bytes(bytes: &Vec<u8>) -> Result<Info, serde_bencode::Error> {
        de::from_bytes::<Info>(&bytes)
    }
    pub fn set_info_hash(&mut self, input: &str){
        self.info_hash = Some(input.to_string());
    }
}

// // todo: macro this shit
impl ToBencode for Info {
    fn to_bencode(&self) -> Bencode {
        // println!{"\n\n\n\nbencoding\n\n\n\n"}
        
        let mut m = BTreeMap::new();

        if self.fileduration.is_some() {
            m.insert(ByteString::from_str("file-duration"), self.fileduration.clone().unwrap().to_bencode());
        }

        if self.filemedia.is_some() {
            m.insert(ByteString::from_str("file-media"), self.filemedia.clone().unwrap().to_bencode());
        }

        if self.ed2k.is_some() {
            m.insert(ByteString::from_str("ed2k"), Bencode::ByteString(self.ed2k.clone().unwrap().into_vec()));
        }

        if self.filehash.is_some() {
            m.insert(ByteString::from_str("filehash"), Bencode::ByteString(self.filehash.clone().unwrap().into_vec()));
        }

        if self.length.is_some(){
            m.insert(ByteString::from_str("length"), self.length.unwrap().to_bencode());
        }
        else if self.files.is_some() {
            let bc = self.files.clone().unwrap().to_bencode();
            m.insert(ByteString::from_str("files"), bc);
        }

        if self.md5sum.is_some() {
            m.insert(ByteString::from_str("md5sum"), self.md5sum.clone().unwrap().to_bencode());
        }

        match &self.name{
            Some(name) => {
                m.insert(ByteString::from_str("name"), name.to_bencode());
            },
            None => {}
        }

        match &self.utf8name{
            Some(name) => {
                m.insert(ByteString::from_str("name.utf-8"), name.to_bencode());
            },
            None => {}
        }

        if self.piece_length.is_some() {
            m.insert(ByteString::from_str("piece length"), self.piece_length.unwrap().to_bencode());
        }

        if self.pieces.is_some() {
            m.insert(ByteString::from_str("pieces"), Bencode::ByteString(self.pieces.clone().unwrap().into_vec()));
        }

        if self.private.is_some() {
            m.insert(ByteString::from_str("private"), self.private.unwrap().to_bencode());
        }

        if self.source.is_some(){
            m.insert(ByteString::from_str("source"), self.source.clone().unwrap().to_bencode());
        }

        Bencode::Dict(m)
    }
}

//TODO fix nodes
#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub info: Info,

    #[serde(default)]
    pub announce: Option<String>,
    
    #[serde(default)]
    nodes: Option<Vec<String>>,
    
    #[serde(default)]
    encoding: Option<String>,
    
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    
    #[serde(default)]
    #[serde(rename="announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    
    #[serde(default)]
    #[serde(rename="creation date")]
    pub creation_date: Option<i64>,
    
    #[serde(rename="comment")]
    comment: Option<String>,
    
    #[serde(default)]
    #[serde(rename="created by")]
    created_by: Option<String>,
}

//TODO: Fix use lifetimes and return reference to hash instead
impl Torrent{
    pub fn new_bytes(input_bytes: &Vec<u8>) ->Result<Torrent, Error> {
        let torrent = de::from_bytes::<Torrent>(&input_bytes)?;
        Ok(torrent)
         
        /* 
            This code will not work correctly since if it does not have a field it will not
            be incorporated into the struct
        
            let mut info = ser::to_bytes::<Info>(&torrent.info)?;
            torrent.info.info_hash = Some(sha1(&info));
            
            /////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////            
            
            this code will regex search the file for the dictionary, hash it,
            and then insert it into the torrent struct. Shit does not currently work
            as part of the file is binary encoded.

            let re = Regex::new("[0-9]:infod").unwrap();
            let result = re.find(&input_bytes);

            match result {
                Some(x_plus_one) => {
                    println!{"\n\n\nmade it"}
                    let info_dict = input_bytes.get(x_plus_one.end()-1..x_plus_one.end()+50);
                        dbg!{String::from_utf8(info_dict.unwrap().to_vec())};

                    match info_dict{
                        Some(to_hash)=>{
                            let hash= sha1(to_hash);
                            torrent.info.info_hash = Some(hash);

                        }
                        None => ()
                    }

                },
                None => ()
            }

            Ok(torrent)
            */

    }
    pub fn new_file(filename: &str) -> Result<Torrent, Error> {
        let mut buffer = Vec::new();
        let mut file = std::fs::File::open(filename).unwrap();
        file.read_to_end(&mut buffer)?;
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
    
    pub fn set_info_hash(&mut self, input: &str) {
        self.info.set_info_hash(input);
    }

}

impl ToBencode for Torrent {
    fn to_bencode(&self) -> Bencode{
        Info::to_bencode(&self.info)
    } 
}


#[derive(Debug, Deserialize)]
pub struct Announce {
    pub complete: i64,
    pub incomplete: i64,
    pub downloaded: i64, 
    pub interval: i64,
    #[serde(default)]
    pub peers: Option<ByteBuf>,
    #[serde(default)]
    pub peers6: Option<ByteBuf>
}

impl Announce {
    pub fn new_bytes(input_bytes: &Vec<u8>) ->Result<Announce, Error> {
        let ann = de::from_bytes::<Announce>(&input_bytes)?;
        Ok(ann)
    }
    pub fn new_file(filename: &str) -> Result<Announce, Error> {
        let mut buffer = Vec::new();
        let mut file = std::fs::File::open(filename)?;
        file.read_to_end(&mut buffer)?;
        Announce::new_bytes(&buffer)
    }
}

pub fn sha1(bytes: &[u8]) -> String {
        let mut hasher = crypto::sha1::Sha1::new();
        hasher.input(&bytes);
        return hasher.result_str();
}

#[derive(Debug)]
pub struct AnnounceComponents {
	pub url : String,
	pub info_hash: String,
	creation_date: i64,
	announce_url: Option<String>,
	interval: Option<i64>,
	last_announce: Option<std::time::Instant>
}

// TODO: fix unwrap
impl AnnounceComponents  {
	pub fn new (url: Option<String>, hash: String, creation_date: Option<i64>) -> Result<AnnounceComponents, Error> {
		// i think this .is_some() is not needed since the outer match
		if url.is_some(){

			let date = match creation_date {
				Some(unix_date) => unix_date,
				//TODO: Log that torrents come without creation dates
				None => utils::get_unix_time()
			};

			Ok(AnnounceComponents {url: url.unwrap(),
								info_hash: hash, 
								creation_date: date,
								announce_url: None,
								interval: None,
								last_announce: None})
		}
		else{
			Err(Error::Torrent(TorrentErrors::NoAnnounceUrl(hash.to_string())))
		}
	}

	// TODO: pass in constructed client for get requests
	pub fn announce(&mut self) -> Result<Announce, Error> {

		// generate an announce url if empty
		if self.announce_url.is_none() {
			let url = url_encoding::Url::new(self.info_hash.to_string(), self.info_hash.to_string());
			let url = url.serialize();

			let mut url_copy = self.url.clone();
			url_copy.push_str("?");
			url_copy.push_str(&url);

			self.announce_url = Some(url_copy);
		}

		
		match &self.announce_url {
			Some(url) => {
				
				// make sure that the tracker is going to let us make an announce call
				if self.last_announce.is_some() {
					let last = self.last_announce.unwrap().elapsed().as_secs() as i64;
					let interval = self.interval.unwrap();
					if last < interval {
						return Err(
							Error::Announce(
								AnnounceErrors::AnnounceNotReady(interval - last)
							))
					}
				}

				// make a get request to the tracker
				match reqwest::get(url) {
					Ok(mut response) => {

						let mut buffer: Vec<u8> = Vec::with_capacity(150);
						response.read_to_end(&mut buffer)?;
						
						let parse = Announce::new_bytes(&buffer)?;
						self.interval = Some(parse.interval);
						self.last_announce = Some(std::time::Instant::now());

						self.configure_next_announce(&parse.complete);

						return Ok(parse);
						
					},

					// there was a problem with the request (most likely the hash)
					Err(_x) => {
						//TODO : log all information on the struct about this error here
						return Err(
							Error::Announce(
								AnnounceErrors::AnnounceUrlError(url.clone())
								)
							)
					}
				}
			}
			// This error should literally never happpen
			// TODO: Log errors here
			None => Err(Error::Announce(AnnounceErrors::AnnounceUrlNone))
		}

	}

    fn configure_next_announce(&mut self, seeds: &i64) {
        let days : i64 = (utils::get_unix_time() - self.creation_date) / 86400;
        let min_seeds : i64= 20; // number of seeds after time period where we check less frequently
        let min_days = 7; // number of days when we check less frequently
        
        let new_interval = 6*60*60;

        if (days < min_days) && (*seeds < min_seeds) {
            self.interval = Some(new_interval);

        }
    }
}