use reqwest;

use rss;
use std::fs;
use std::io::prelude::*;
use crate::read_torrent::Torrent;
use super::url_encoding;

#[derive(Debug)]
pub enum Error{
	IO(std::io::Error),
	Reqwest(reqwest::Error),
	Rss(RssErrors),
	UrlError,
	torrent(TorrentErrors),
	Announce(AnnounceErorrs)
}
impl From<reqwest::Error> for Error{
	fn from(error: reqwest::Error) -> Error{
		return Error::Reqwest(error)
	}
}
impl From<std::io::Error> for Error{
	fn from(error: std::io::Error) -> Error{
		return Error::IO(error)
	}
}
impl From<rss::Error> for Error {
	fn from(error: rss::Error) -> Error{
		return Error::Rss(RssErrors::RawRssError(error))
	}
}
impl From<serde_bencode::Error> for Error {
	fn from(error: serde_bencode::Error) ->Error {
		return Error::torrent(TorrentErrors::SerdeError(error))
	}
}
impl From<serde_urlencoded::ser::Error> for Error {
	fn from(error: serde_urlencoded::ser::Error) -> Error {
		return Error::Announce(AnnounceErorrs::SerdeError(error))
	}
}
#[derive(Debug)]
pub enum AnnounceErorrs{
	SerdeError(serde_urlencoded::ser::Error)
}

#[derive(Debug)]
pub enum RssErrors {
	RawRssError(rss::Error),
	InfoHashFetch(&'static str)
}
#[derive(Debug)]
pub enum TorrentErrors {
	NoAnnounceUrl,
	SerdeError(serde_bencode::Error)
}

pub fn get_xml(url: &str) -> Result<(), Error> {

	//TODO: move this to a lazy_static!{}
	let temp_folder : &'static str = r"C:\Users\Brooks\github\nyaa_tracker\temp";
	let tmp = fs::create_dir(temp_folder);	

	let path : &'static str = r"C:\Users\Brooks\github\nyaa_tracker\temp\temp.xml";

	// let mut req = reqwest::get(url)?;
	// let xml_data = reqwest::get(url)?.text()?;


	// let mut file = fs::File::create(path)?;
	// file.write_all(xml_data.as_bytes());
	

	let file = fs::File::open(path)?;
	let channel = rss::Channel::read_from(std::io::BufReader::new(file))?;
	let items = channel.items();


	let mut all_data: Vec<AnnounceComponents>= Vec::with_capacity(items.len());

	for i in items{
		dbg!{nyaa_hash_from_xml(&i)};
		// let saer: i32 = i.extensions().get("nyaa").unwrap().get("infoHash").unwrap()[0].;

		// TODO: better handling for bad requests
		match download_torrent(i.link()){
			Ok(x) => {

				match nyaa_hash_from_xml(&i) {
					Ok(info_hash) => {
						match AnnounceComponents::new(x.announce, info_hash) {
							Ok(announce)=> all_data.push(announce),
							Err(x) => println!{"there was an error with the annouce struct: {:?}", x}
						}
						
					},
					Err(x) => println!{"infohash was no ok : {:?}", x}
				}
			},
			Err(x) => println!{"there was an error with torrent link: {:?}", x}
		}
		break;

	}
	dbg!{all_data};
	return Ok(())
}

#[derive(Debug)]
pub struct AnnounceComponents <'a> {
	pub url : String,
	pub info_hash: &'a str,
	announce_url: Option<String>
}

// TODO: fix unwrap
impl <'a> AnnounceComponents<'a> {
	pub fn new (url: Option<String>, hash: &'a str) -> Result<AnnounceComponents, Error> {
		if url.is_some(){
			let url = url.unwrap();
			Ok(AnnounceComponents {url: url, info_hash: hash, announce_url: None})
		}
		else{
			Err(Error::torrent(TorrentErrors::NoAnnounceUrl))
		}
	}
	pub fn announce(&mut self) -> Result<String, Error> {
		if self.announce_url.is_none() {
			let url = url_encoding::Url::new(self.info_hash.to_string(), self.info_hash.to_string());
			let url = serde_urlencoded::to_string(url)?;
			let mut url_copy = self.url.clone();
			url_copy.push_str("?");
			url_copy.push_str(&url);
			self.announce_url = Some(url_copy);
		}
		match &self.announce_url {
			Some(url) => {
				println!{"the url being queried is"};
				dbg!{&url};
				let response = reqwest::get(url);
				dbg!{response};	

			}
			None => ()
		}
		// let announce_url = &self.announce_url.unwrap();
		

		return Ok("".to_string())
	}
}

// do this since ? does not work w/ Option<T>
fn nyaa_hash_from_xml<'a> (item: &'a rss::Item) -> Result<&'a str, Error>{
	let ext = item.extensions();

	match ext.get("nyaa"){
		Some(nyaa) => {
			match nyaa.get("infoHash") {
				Some(extension_vec) => {
					if extension_vec.len() ==1{
						let ext_index = &extension_vec[0];
						match ext_index.value() {
							Some(infohash) => {
								return Ok(infohash)
							}
							None => Err(Error::Rss(RssErrors::InfoHashFetch("No value field")))
						}
					}
					else {
						Err(Error::Rss(RssErrors::InfoHashFetch("!= one item in the vector")))
					}
				}
				None => Err(Error::Rss(RssErrors::InfoHashFetch("no field infohash")))
			}
		}
		None => Err(Error::Rss(RssErrors::InfoHashFetch("No field nyaa")))
	}
}


// TODO: configure client pooling
// probably want to turn this thing into a struct
pub fn download_torrent(url: Option<&str>) -> Result<Torrent, Error> {
	dbg!{url};
	if url.is_some(){
		let raw_url = url.unwrap();
		let mut buffer: Vec<u8> = Vec::with_capacity(10_000);
		let k = reqwest::get(raw_url)?.read_to_end(&mut buffer)?;
		
		write_torrent_to_file(&raw_url, &buffer);
		let t = Torrent::new_bytes(&buffer);
	
		Ok(t?)
	}
	else{
		Err(Error::UrlError)
	}
	
}


pub fn write_torrent_to_file(url: &str, data: &Vec<u8>) -> String {
	let mut file_path: String = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
	
	let mut last = 0;
	println!{"here"}
	for i in 0..url.len()-1 {
		if url.get(i..i+1).unwrap() == "/" {
			last = i;
		}
	}

	let filename = &url.get(last+1..url.len()).unwrap();
	file_path.push_str(&filename);
	
	dbg!{&file_path};

	let mut file = std::fs::File::create(&file_path).unwrap();
	file.write_all(&data);


	return file_path
}



pub fn compare_files(f1: &str, f2: &str) -> () {

    let mut buffer1 = Vec::new();
    let mut file1 = std::fs::File::open(f1).unwrap();
    file1.read_to_end(&mut buffer1);

    let mut buffer2 = Vec::new();
    let mut file2 = std::fs::File::open(f2).unwrap();
    file2.read_to_end(&mut buffer2);


    println!{"f1 len:\t{}\tf2 len:\t{}",buffer1.len(), buffer2.len()}

    for i in 0..1000 {
        let c1 = &buffer1[i];
        let c2 = &buffer2[i];
        if c1 == c2 {
            continue
        }
        else{
            println!{"{} {} {}", i, buffer1[i], buffer2[i]}
        }
    } //for
}