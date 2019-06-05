use reqwest;

use rss;
use std::fs;
use std::io::prelude::*;
use super::super::read::{announce_components::*, announce_result::*, torrent::*};
// use crate::read_torrent::{Torrent, Announce, AnnounceComponents};
use super::url_encoding;

use hashbrown::HashSet;

use super::super::utils;

use super::super::error::*;


macro_rules! parse {
	($func:ident, $parse_item:ident, $good_data:ident, $error_data:ident, $previous:ident) => {
		match $func(&$parse_item){
			Ok(info_hash) => {

				if $previous.contains(info_hash) {
					println!{"skipping torrent {}", info_hash}
					continue
				}
				else {
					$previous.insert(info_hash.to_string());
				}
				match utils::download_torrent($parse_item.link(), &info_hash) {
					Ok(torrent) => {
						match AnnounceComponents::new(torrent.announce, info_hash.to_string(), torrent.creation_date, torrent.info.name().unwrap()){
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
pub fn get_xml(url: &str, previous: &mut HashSet<String>) -> Result<Data, Error> {


	//TODO: move this to a lazy_static!{}
	println!{"creating temp folder"}
	let temp_folder : &'static str = r"C:\Users\Brooks\github\nyaa_tracker\temp";
	fs::create_dir(temp_folder);

	let mut path: String = temp_folder.to_string();
	path.push_str(r"\");
	path.push_str(&utils::get_unix_time().to_string());
	path.push_str(".xml");

	println!{"writing to file"}
	// request xml data and read to file
	let xml_data = reqwest::get(url)?.text()?;
	let mut file = fs::File::create(&path)?;
	file.write_all(xml_data.as_bytes())?;
	
	// read xml data from file
	let file = fs::File::open(&path)?;
	let channel = rss::Channel::read_from(std::io::BufReader::new(file))?;
	let mut items = channel.items().to_vec();
	
	// storage vectors
	let mut good_data: Vec<AnnounceComponents>= Vec::with_capacity(items.len());
	let mut error_data: Vec<Error> = Vec::new();

	for _ in 0..items.len() {

		let current_item = items.remove(0);

		if url.contains(".si"){
			parse!(nyaa_si_hash, current_item, good_data, error_data, previous)
		}
		else if url.contains("pantsu.cat"){
			parse!(nyaa_pantsu_hash, current_item, good_data, error_data, previous)
		}
		else {
			panic!("RSS url is not correct. fix that shit")
		}

	}

	return Ok(Data::new(good_data, error_data))
}

#[derive(Debug)]
pub struct Data  {
	pub good : Vec<AnnounceComponents>,
	pub bad : Vec<Error>
}
impl Data{
	fn new(good: Vec<AnnounceComponents>, bad : Vec<Error>) -> Data{ 
		Data {good: good, bad: bad}
	}
}


// do this since ? does not work w/ Option<T>
fn nyaa_si_hash <'a>(item: &'a rss::Item) -> Result<&'a str, Error>{
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


fn nyaa_pantsu_hash<'a>(item: &'a rss::Item) -> Result<&'a str, Error> {
	let link = item.link();
	match link {
		Some(data) =>{
			return utils::content_after_last_slash(&data)
		},
		None => return Err(Error::Rss(RssErrors::CouldNotReadRss))
	};
}