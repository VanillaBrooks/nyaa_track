use reqwest;

use rss;
use std::fs;
use std::io::prelude::*;
use crate::read_torrent::{Torrent, Announce};
use super::url_encoding;

use super::super::utils;

use super::super::error::*;
// use super::super::error::{Error, AnnounceErrors, RssErrors, TorrentErrors };
macro_rules! parse {
	($func:ident, $parse_item:ident, $good_data:ident, $error_data:ident) => {
		match $func(&$parse_item){
			Ok(info_hash) => {
				match utils::download_torrent($parse_item.link(), &info_hash) {
					Ok(Torrent) => {
						match AnnounceComponents::new(Torrent.announce, info_hash, Torrent.creation_date){
							Ok(announce) => $good_data.push(announce),
							Err(announce_err) => $error_data.push(announce_err) // store annouce error
						}
					},
					Err(link_error) => $error_data.push(link_error)// store link error
				}
			},
			Err(error) => $error_data.push(error)
		}
	};
}

// download xml data from a url as well as their associated torrents
// return a vector of structs required to make announcements
// will only Error if the provided url is incorrect
pub fn get_xml(url: &str) -> Result<Data, Error> {

	//TODO: move this to a lazy_static!{}
	let temp_folder : &'static str = r"C:\Users\Brooks\github\nyaa_tracker\temp";
	fs::create_dir(temp_folder);

	let mut path: String = temp_folder.to_string();
	path.push_str(r"\");
	path.push_str(&utils::get_unix_time().to_string());
	path.push_str(".xml");

	// request xml data and read to file
	let xml_data = reqwest::get(url)?.text()?;
	let mut file = fs::File::create(&path)?;
	file.write_all(xml_data.as_bytes());
	
	// read xml data from file
	let file = fs::File::open(&path)?;
	let channel = rss::Channel::read_from(std::io::BufReader::new(file))?;
	let mut items = channel.items().to_vec();
	

	let mut good_data: Vec<AnnounceComponents>= Vec::with_capacity(items.len());
	let mut error_data: Vec<Error> = Vec::new();

	for i in 0..items.len() {
		println!{"{}", i}
		let current_item = items.remove(0);
		// dbg!{nyaa_hash_from_xml(current_item)};

		if url.contains(".si"){
			parse!(nyaa_si_hash, current_item, good_data, error_data)
		}
		// else if url.contains("pantsu.cat"){
		// 	parse!(nyaa_pantsu_hash,current_item)
		// }
	}

	return Ok(Data::new(good_data, error_data))
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
impl AnnounceComponents {
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
			Err(Error::Torrent(TorrentErrors::NoAnnounceUrl(hash)))
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

#[derive(Debug)]
pub struct Data {
	pub good : Vec<AnnounceComponents>,
	pub bad : Vec<Error>
}
impl Data {
	fn new(good: Vec<AnnounceComponents>, bad : Vec<Error>) -> Data{ 
		Data {good: good, bad: bad}
	}
}


// do this since ? does not work w/ Option<T>
fn nyaa_si_hash(item: &rss::Item) -> Result<String, Error>{
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


fn nyaa_pantsu_hash(item: &rss::Item) -> Result<String, Error> {
	unimplemented!()
}