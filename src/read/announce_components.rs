use super::super::error::*;
use super::super::utils;
use super::super::requests::url_encoding;

use std::io::prelude::*;
use std::time::Duration;

use super::{AnnounceResult, ScrapeResult, ScrapeData, GenericData};

use std::sync::Arc;

use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use hyper::rt::{Future, Stream};

use tokio::timer::Timeout;

#[derive(Debug, Clone)]
pub struct AnnounceComponents {
	pub url : Arc<String>,
	pub info_hash: Arc<String>,
	pub title: Arc<String>,
	pub creation_date: Arc<i64>,
	announce_url: hyper::Uri,
	scrape_url: hyper::Uri,
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
			
			Ok(AnnounceComponents {url: Arc::new(url),
								info_hash: Arc::new(hash), 
								creation_date: Arc::new(date),
								title: Arc::new(title),
								announce_url: ann_url,
								scrape_url: scrape_url,
								client: utils::https_connection(4)})
		}
		else{
			Err(Error::Torrent(TorrentErrors::NoAnnounceUrl(hash.to_string())))
		}
	}

	pub fn get(
		self,
		tx_announce: mpsc::Sender<AnnounceComponents>,
		tx_database: mpsc::Sender<GenericData>
		) -> () {
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

								// dbg!{"scrape data acquired"};

								if self.allow_future_scrapes(&data.complete){
									tx_announce.send(self).wait();
								}
								else{
									println!{"dropping item"}
								}

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


		let fut = 
		Timeout::new(request , Duration::from_secs(10))
				.map(move |x| {
					// println!{"success in scrape data"}
					tx_database.send(x).wait();
					})
				.map_err(|_| ());

		tokio::spawn(fut);

	}

	fn allow_future_scrapes(&self, seeders: &i64) -> bool {
		let days_alive = (utils::get_unix_time() - self.creation_date) / 86400;

		// older than 7 days, less than 100 active seeders we terminate tracking
		if days_alive > 7 && *seeders < 100 {
			false
		}
		else {
			true
		}
	}
	
}