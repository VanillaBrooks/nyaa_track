use super::super::error::*;
use super::super::utils;
use super::super::requests::url_encoding;

use std::io::prelude::*;

use super::{AnnounceResult, ScrapeResult, ScrapeData, GenericData};


#[derive(Debug)]
pub struct AnnounceComponents {
	pub url : String,
	pub info_hash: String,
	pub title: String,
	creation_date: i64,
	announce_url: Option<String>,
	interval: Option<i64>,
	last_announce: Option<std::time::Instant>
}

// TODO: fix unwrap
impl AnnounceComponents  {
	pub fn new (url: Option<String>, hash: String, creation_date: Option<i64>, title: String) -> Result<AnnounceComponents, Error> {
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
								title: title,
								announce_url: None,
								interval: None,
								last_announce: None})
		}
		else{
			Err(Error::Torrent(TorrentErrors::NoAnnounceUrl(hash.to_string())))
		}
	}

	// TODO: pass in constructed client for get requests
	// pub fn announce(&mut self) -> Result<AnnounceResult, Error> {

	// }

    fn configure_next_announce(self: &Self, seeds: &i64) -> Option<i64> {
        let days : i64 = (utils::get_unix_time() - self.creation_date) / 86400;
        let min_seeds : i64= 20; // number of seeds after time period where we check less frequently
        let min_days = 7; // number of days when we check less frequently
        
        let new_interval = 6*60*60;

        if (days < min_days) && (*seeds < min_seeds) {
			return Some(new_interval)
        }
		else {
			None
		}
    }
}
impl PullData for AnnounceComponents {
	fn run(&mut self) -> Result<GenericData, Error> {
		
		// generate an announce url if empty
		if self.announce_url.is_none() {
			// get all the extentions ?asd=234235&asda=234 extensions
			let url = url_encoding::AnnounceUrl::new(self.info_hash.to_string(), self.info_hash.to_string());
			let url = url.serialize();

			//push all the extensions onto the base url
			let mut url_copy = self.url.clone();
			url_copy.push_str("?");
			url_copy.push_str(&url);

			self.announce_url = Some(url_copy);

			dbg!{&self.announce_url};
			panic!{"119 line in ann_comp "};
		}


		// run the announce
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
						
						let parse = AnnounceResult::new_bytes(&buffer, &self.info_hash, &self.url, &self.title, &self.creation_date)?;
						if parse.data.interval.is_some() {
							self.interval = parse.data.interval
						}
						self.last_announce = Some(std::time::Instant::now());

						// update the next announce interval
						match self.configure_next_announce(&parse.data.complete){
							Some(new_interval) => self.interval = Some(new_interval),
							None => ()
						}

						return Ok(GenericData::new(&self.info_hash,
												&self.url,
												&self.creation_date,
												&self.title,
												parse.data.downloaded,
												parse.data.complete,
												parse.data.incomplete))
						
					},

					// there was a problem with the request (most likely the hash)
					Err(_x) => {
						dbg!{_x};
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

pub struct ScrapeComponents {
	pub url : String,
	pub info_hash: String,
	pub title: String,
	creation_date: i64,
	scrape_url: Option<String>,
}

impl ScrapeComponents {
	pub fn new(url: Option<String>, hash: String, creation_date: Option<i64>, title: String) -> Result<ScrapeComponents, Error>{
		if url.is_some() {
			let date = match creation_date {
				Some(date) => date,
				None => utils::get_unix_time()
			};
			
			Ok(ScrapeComponents {
				url: url.unwrap(),
				info_hash: hash,
				title: title,
				creation_date: date,
				scrape_url: None
			})

		}
		else {
			Err(Error::UrlError)
		}
	}
}

impl PullData for ScrapeComponents {
	fn run(&mut self) -> Result<GenericData, Error> {
		if self.scrape_url.is_none() {
			let url_struct = url_encoding::ScrapeUrl::new(vec![&self.url]);

			self.scrape_url = Some(url_struct.announce_to_scrape(&self.url)?);
		}

		match &self.scrape_url {
			Some(url) => {
				match reqwest::get(url) {
					Ok(mut response) =>{
						
						let mut buffer: Vec<u8> = Vec::with_capacity(150);
						response.read_to_end(&mut buffer)?;
						
						let parse = ScrapeData::new_bytes(&buffer)?;

						match parse.files.values().into_iter().next() {
							Some(data) => {

								Ok(GenericData::new(&self.info_hash,
													&self.url,
													&self.creation_date,
													&self.title,
													data.downloaded,
													data.complete,
													data.complete))

							}
							None => { // todo: log this error this should not happen
										// the file did not serialize the mandatory field
								Err(Error::ShouldNeverHappen("the fields of scrapedata were not correctly filled".to_string()))
							}
						}

					}
					Err(error) => { // TODO: log this error
						Err(
							Error::Announce(
								AnnounceErrors::AnnounceUrlError(url.clone())
								)
							)
					}
				}

			}
			None => { //TODO log this error
				Err(Error::UrlError)
			}
		}
	}
}


pub trait PullData {
	fn run(&mut self) -> Result<GenericData, Error> ;
}

pub enum DataFormat <'a> {
	individual(Result<GenericData<'a>, Error>),
	vector(Vec<Result<GenericData<'a>, Error>>)
}