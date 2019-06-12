use super::super::error::*;
use super::super::utils;
use super::super::requests::url_encoding;

use std::io::prelude::*;

use super::{AnnounceResult, ScrapeResult, ScrapeData, GenericData};

use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use hyper::rt::{Future, Stream};

#[derive(Debug)]
pub struct AnnounceComponents {
	pub url : String,
	pub info_hash: String,
	pub title: String,
	creation_date: i64,
	announce_url: hyper::Uri,
	scrape_url: hyper::Uri,
	interval: Option<i64>,
	last_announce: Option<std::time::Instant>,
	client: Client<HttpsConnector<HttpConnector>>
}

// TODO: fix unwrap
impl <'a>AnnounceComponents  {
	pub fn new (url: Option<String>, hash: String, creation_date: Option<i64>, title: String) -> Result<AnnounceComponents, Error> {
		// i think this .is_some() is not needed since the outer match
		if url.is_some(){
			let url = url.unwrap();

			let date = match creation_date {
				Some(unix_date) => unix_date,
				//TODO: Log that torrents come without creation dates
				None => utils::get_unix_time()
			};

			//TODO: Fix this mess

			// announce_url calculation
			let url_ = url_encoding::AnnounceUrl::new(hash.to_string(), hash.to_string());
			let url_ = url_.serialize();

			//push all the extensions onto the base url
			let mut ann_url = url.clone();
			ann_url.push_str("?");
			ann_url.push_str(&url);
			let ann_url = ann_url.parse()?;

			// scrape url calc
			let url_struct = url_encoding::ScrapeUrl::new(&hash);
			let scrape_url = url_struct.announce_to_scrape(&url)?.parse()?;
			
			Ok(AnnounceComponents {url: url,
								info_hash: hash, 
								creation_date: date,
								title: title,
								announce_url: ann_url,
								scrape_url: scrape_url,
								interval: None,
								last_announce: None,
								client: utils::https_connection(4)})
		}
		else{
			Err(Error::Torrent(TorrentErrors::NoAnnounceUrl(hash.to_string())))
		}
	}
/*
	fn announce(self: &'a mut Self) -> Result<GenericData<'a>, Error> {
		
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
		match reqwest::get(self.announce_url) {
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
*/
	pub fn get(self: &'a mut Self, url: hyper::Uri) -> impl Future<Item=GenericData<'a>, Error=Error> {
		let client = Client::new();

		client
			// Fetch the url...
			.get(self.scrape_url.clone())
			// And then, if we get a response back...
			.and_then(|res| {
				// asynchronously concatenate chunks of the body
				res.into_body().concat2()
			})
			.from_err::<Error>()
			.and_then(move |body| {
				let data = body.into_bytes().into_iter().collect::<Vec<_>>();
				// ScrapeData::new_bytes(&data)
				match ScrapeData::new_bytes(&data) {
					Ok(scrape) => {
						match scrape.files.values().into_iter().next(){
							Some(data) => {
								Ok(GenericData::new(
									&self.info_hash,
									&self.url,
									&self.creation_date,
									&self.title,
									data.downloaded,
									data.complete,
									data.incomplete)
								)
							}
							None => Err(Error::ShouldNeverHappen("the fields of scrapedata were not correctly filled".to_string()))
						}
					},
					Err(_) => Err(Error::UrlError)
				}
			})
	}

}
#[derive(Debug)]
enum FetchError {
    Http(hyper::Error),
    // Json(serde_json::Error),
    Error
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}
impl From<Error> for FetchError{
    fn from(err: Error) -> FetchError {
        FetchError::Error
    }
}