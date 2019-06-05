use super::super::error::*;
use super::super::utils;
use super::super::requests::url_encoding;

use serde_bencode::{self, de};
use serde_derive::{Deserialize};
use serde_bytes::{self, ByteBuf};
use std::fs;
use std::io::prelude::*;

use super::announce_result::AnnounceResult;

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
	pub fn announce(&mut self) -> Result<AnnounceResult, Error> {

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
						
						let parse = AnnounceResult::new_bytes(&buffer, self.info_hash.clone(), self.url.clone(), self.title.clone(), self.creation_date)?;
						self.interval = Some(parse.data.interval);
						self.last_announce = Some(std::time::Instant::now());

						self.configure_next_announce(&parse.data.complete);

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