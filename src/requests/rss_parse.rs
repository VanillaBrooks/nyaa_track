// use reqwest;
use hyper::rt::{Future, Stream};
use hyper::client::{Client, HttpConnector};
use futures::sync::mpsc;
use tokio::timer::Timeout;

use std::sync::Arc;

use rss;
use std::fs;
use std::io::prelude::*;
use std::time::{self, Duration};
use std::thread;
use super::super::read::{Torrent, AnnounceComponents};

use hashbrown::HashSet;

use super::super::utils;

use super::super::error::*;

use parking_lot::RwLock;

macro_rules! parse {
	($parse_funct:ident, $parse_item:ident, $previous:ident, $dl:ident, $tx:ident) => {
		match $parse_funct(&$parse_item){
			Ok(info_hash) => {
				
				// make sure we are not grabbing an old hash
				{
					let previous = $previous.read();
					if previous.contains(info_hash) {continue}
				}
				// if we are about to download the torrent sleep it for 1 second 
				// prevents tracker from blocking us
				thread::sleep(Duration::from_millis(300));

				// make sure the link is good
				match $parse_item.link(){
					Some(good_url) => {
						let tx = $tx.clone();
						//create a downloading future
						let download_fut = Timeout::new(
							$dl.download(
								good_url, 
								info_hash.to_string(), 
								tx
								),
							Duration::from_secs(5))		// each future has 5 seconds to complete)
							.map(|x| println!{"recieved good torrent data!"})
							.map_err(|x| println!{"Error downloading torrent data"});
					tokio::spawn(download_fut);
					}

					None => println!{"error with link"}
				}
				
			},
			Err(error) => {
				println!{"error with RSS item"};
			}
		}
	};
}


/*
	TODO: add config for destination folders
 */

// download xml data from a url as well as their associated torrents
// return a vector of structs required to make announcements
// will only Error if the provided url is incorrect
pub fn get_xml<'a>(
	url: &str, 
	previous: Arc<RwLock<HashSet<String>>>, 
	tx_to_filter: mpsc::Sender<AnnounceComponents>
	// ) -> Result<Data, Error> {
	) -> impl Future<Item=(), Error=Error> + 'a {

	// decide what hash-parsing function we will use for the given url
	let parse_funct = 
		if url.contains(".si") {nyaa_si_hash}
		else {nyaa_pantsu_hash};

	let client = utils::https_connection(10);
	let uri = url.parse().expect("rss url invalid");

	dbg!{"in get_xml"};

	client.get(uri)
		.and_then(|res| res.into_body().concat2())
		.from_err::<Error>()
		.and_then(|data| {
			let data = data.into_bytes().into_iter().collect::<Vec<u8>>();

			// create XML file path
			let mut path: String = r".\temp".to_string();
			path.push_str(r"\");
			path.push_str(&utils::get_unix_time().to_string());
			path.push_str(".xml");

			// request xml data and read to file
			let mut file = fs::File::create(&path).expect("xml file could not be created");
			file.write_all(&data);
			
			// read xml data from file
			let file = fs::File::open(&path).expect("file could not be opened");
			let channel = rss::Channel::read_from(std::io::BufReader::new(file)).expect("error when reading rss");
			let items = channel.into_items().to_vec();

			std::fs::remove_file(path);

			let dl = utils::Downloader::new();

			Ok((items, dl))
		})
		.and_then(move |(mut items, dl)| {

			for _ in 0..items.len() {

				let item = items.remove(0);

				parse!{parse_funct, item, previous, dl, tx_to_filter};

			}
			Ok(())
			
		})

}


// timer for rss updates
pub struct Timer <'a> {
	last_check: time::Instant,
	time_between: u32, // in seconds
	pub url: &'a str
}
impl <'a>Timer <'a> {
	pub fn new(between: u32, url: &'a str) -> Timer<'a> {
		Timer{last_check: time::Instant::now(),
			  time_between: between,
			  url: url}
	}
	pub fn allow_check(&mut self) -> bool {
		let now = time::Instant::now();
		let elapsed = (now - self.last_check).as_secs() as u32;
		if elapsed > self.time_between {
			self.last_check = now;
			true
		}
		else {
			false
		}
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