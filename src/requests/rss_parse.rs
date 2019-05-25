use reqwest;

use rss;
use std::fs;
use std::io::prelude::*;
use crate::read_torrent::{Torrent, Announce};
use super::url_encoding;

use super::super::utils;

use super::super::error::*;
// use super::super::error::{Error, AnnounceErrors, RssErrors, TorrentErrors };

// download xml data from a url as well as their associated torrents
// return a vector of structs required to make announcements
// will only Error if the provided url is incorrect
pub fn get_xml(url: &str) -> Result<Vec<AnnounceComponents>, Error> {

	//TODO: move this to a lazy_static!{}
	let temp_folder : &'static str = r"C:\Users\Brooks\github\nyaa_tracker\temp";
	fs::create_dir(temp_folder);

	let mut path: String = temp_folder.to_string();
	path.push_str(r"\");
	path.push_str(&utils::get_unix_time().to_string());
	path.push_str(".xml");

	let mut req = reqwest::get(url)?;
	let xml_data = reqwest::get(url)?.text()?;


	let mut file = fs::File::create(&path)?;
	file.write_all(xml_data.as_bytes());
	

	let file = fs::File::open(&path)?;
	let channel = rss::Channel::read_from(std::io::BufReader::new(file))?;
	let mut items = channel.items().to_vec();
	


	let mut good_data: Vec<AnnounceComponents>= Vec::with_capacity(items.len());
	let mut error_data: Vec<Error> = Vec::new();

	for i in 0..items.len() {
		println!{"{}", i}
		let current_item = items.remove(0);
		// dbg!{nyaa_hash_from_xml(current_item)};


		// TODO: better handling for bad requests
		match utils::download_torrent(current_item.link()){
			Ok(x) => {

				match nyaa_hash_from_xml(current_item) {
					Ok(info_hash) => {
						match AnnounceComponents::new(x.announce, info_hash) {
							Ok(announce)=> good_data.push(announce),
							Err(x) => {
								println!{"there was an error with the announce struct: {:?}", x};
								error_data.push(x);
							}	
						}
						
					},
					Err(error) => {
						println!{"infohash was no ok : {:?}", error};
						error_data.push(error);
					}
				}
			},

			//TODO: handle this error better
			Err(x) => {
				println!{"there was an error with torrent link: {:?}", x};
				error_data.push(x);
			}
		}
	}
	println!{"all data: "}
	dbg!{&good_data};
	dbg!{error_data};
	return Ok(good_data)
}

#[derive(Debug)]
pub struct AnnounceComponents {
	pub url : String,
	pub info_hash: String,
	announce_url: Option<String>,
	interval: Option<u64>,
	last_announce: Option<std::time::Instant>
}

// TODO: fix unwrap
impl AnnounceComponents {
	pub fn new (url: Option<String>, hash: String) -> Result<AnnounceComponents, Error> {
		// i think this .is_some() is not needed since the outer match
		if url.is_some(){

			Ok(AnnounceComponents {url: url.unwrap(),
								info_hash: hash, 
								announce_url: None,
								interval: None,
								last_announce: None})
		}
		else{
			Err(Error::Torrent(TorrentErrors::NoAnnounceUrl(hash)))
		}
	}


	pub fn announce(&mut self) -> Result<Announce, Error> {
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
				
				// make sure that the tracker is going to let us make an announce cal
				if self.last_announce.is_some() {
					let last = self.last_announce.unwrap().elapsed().as_secs();
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

						return Ok(parse);
						
					},

					// there was a problem with the request (most likely the hash)
					Err(x) => {
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
}

// do this since ? does not work w/ Option<T>
fn nyaa_hash_from_xml(item: rss::Item) -> Result<String, Error>{
	let ext = item.extensions();

	match ext.get("nyaa"){
		Some(nyaa) => {
			match nyaa.get("infoHash") {
				Some(extension_vec) => {
					if extension_vec.len() ==1{
						let ext_index = &extension_vec[0];
						match ext_index.value() {
							Some(infohash) => {
								return Ok(infohash.to_string())
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


