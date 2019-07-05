use super::super::error::*;
use super::super::utils;
use super::super::requests::url_encoding;

use std::io::prelude::*;
use std::time::Duration;

use super::{ScrapeData, GenericData};
use futures::sync::mpsc;
use futures::sync::oneshot;
use futures::Sink;
use futures::future;
use future::lazy;

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


			// scrape url calc
			let url_struct = url_encoding::ScrapeUrl::new(&hash);
			let scrape_url = url_struct.announce_to_scrape(&url)?.parse()?;
			
			Ok(AnnounceComponents {url: Arc::new(url),
								info_hash: Arc::new(hash), 
								creation_date: Arc::new(date),
								title: Arc::new(title),
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

		let hash = self.info_hash.clone();
		let hash_clone = self.info_hash.clone();
		let url = self.url.clone();
		let creation_date = self.creation_date.clone();
		let title = self.title.clone();


		let request = self.client
			// Fetch the url...
			.get(self.scrape_url.clone())
			// And then, if we get a response back...
			.and_then(|res| {
				// asynchronously concatenate chunks of the body
				res.into_body().concat2()
			})
			.from_err::<Error>()
			.and_then(move |body| {
				// dbg!{"getting data"};
				let data = body.into_bytes().into_iter().collect::<Vec<_>>();

				match ScrapeData::new_bytes(&data) {
					Ok(scrape) => {

						// turn the parsed dictionary into an iterator
						match scrape.files.values().into_iter().next(){
							Some(data) => {


								let gen_data = 
									GenericData::new(
										hash,
										url,
										creation_date,
										title,
										data.downloaded,
										data.complete,
										data.incomplete);

								Ok(gen_data)

							}
							None => {
								let e = Err(
									Error::ShouldNeverHappen(
										format!{"the fields of scrapedata for {:?} were not correctly filled", &hash_clone}
									)
								);
								e
							}
						}

					},
					Err(_) => {
						Err(Error::UrlError)
					}
				}
			});

		let self_clone = self.clone();
		let tx_announce_clone = tx_announce.clone();

		let fut = 
		Timeout::new(request , Duration::from_secs(10))
				.map(move |x| {
					if self.allow_future_scrapes(&x.complete) {
						tx_announce.send(self).wait();
					}
					else{
						println!{"dropped item"}
					}
					tx_database.send(x).wait();

					})
				.map_err(move |error| {

					println!{"timeout error : {:?}", &error}
					tx_announce_clone.send(self_clone).wait();

				});

		tokio::spawn(fut);

	}

	fn allow_future_scrapes(&self, seeders: &i64) -> bool {
		let creation_data_ptr = Arc::into_raw(self.creation_date.clone());
		let creation_date = unsafe{*creation_data_ptr};
		
		unsafe {Arc::from_raw(creation_data_ptr) };

		let days_alive = (utils::get_unix_time() - creation_date) / 86400;

		// older than 7 days, less than 100 active seeders we terminate tracking
		if days_alive > 7 && *seeders < 100 {
			false
		}
		else {
			true
		}
	}
	
}