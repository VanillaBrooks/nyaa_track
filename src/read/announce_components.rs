use super::super::error::*;
use super::super::utils;
use super::super::requests::url_encoding;

use std::io::prelude::*;

use super::{AnnounceResult, ScrapeResult, ScrapeData, GenericData};

use futures::sync::mpsc;

use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use hyper::rt::{Future, Stream};

#[derive(Debug, Clone)]
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
	pub fn new (
		url: Option<String>, 
		hash: String, 
		creation_date: Option<i64>, 
		title: String
		) -> Result<AnnounceComponents, Error> {

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

	pub fn get(
		self: &Self,
		) -> impl Future<Item=GenericData, Error=Error> {
		// ) -> () {
		let client = Client::new();


		let hash = self.info_hash.clone();
		let url = self.url.clone();
		let creation_date = self.creation_date.clone();
		let title = self.title.clone();

		let mut request = client
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

				match ScrapeData::new_bytes(&data) {
					Ok(scrape) => {

						// turn the parsed dictionary into an iterator
						match scrape.files.values().into_iter().next(){
							Some(data) => {
								let gen_data = 
									GenericData::new(
										hash,					//TODO: make these RC values
										url,
										creation_date,
										title,
										data.downloaded,
										data.complete,
										data.incomplete);
								
								Ok(gen_data)
							}
							None => Err(Error::ShouldNeverHappen("the fields of scrapedata were not correctly filled".to_string()))
						}

					},
					Err(_) => Err(Error::UrlError)
				}
			});
		request
		// let fut = request
		// 	.map(|x| println!{"success in scrape data"})
		// 	.map_err(|x| println!{"there was an error with the scrape"});
		// tokio::spawn(fut);
	}
}

struct Move{
	test: String
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