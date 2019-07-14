use super::super::error::*;
use super::super::error::HTTPErrors;
use super::super::utils;
use super::super::requests::url_encoding;

use std::io::prelude::*;
use std::time::Duration;

use super::{ScrapeData, AnnounceData, GenericData};
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

enum RequestType{
	Announce((GenericData, i64)),
	Scrape(GenericData)
}



#[derive(Debug, Clone)]
pub struct AnnounceComponents {
	pub url : Arc<String>,
	pub info_hash: Arc<String>,
	pub title: Arc<String>,
	pub creation_date: Arc<i64>,
	scrape_url: hyper::Uri,
	announce_url: hyper::Uri,
	client: Client<HttpsConnector<HttpConnector>>,
	scrape_error_count: i64,
	announce_error_count: i64,
	incomplete_data: i64,
	next_announce: i64,
	struct_initialization_time: i64
}

// TODO: fix unwrap
impl <'a>AnnounceComponents  {
	pub fn new (
		url: Option<String>, 
		hash: String, 
		creation_date: Option<i64>, 
		title: String
		) -> Result<AnnounceComponents, Error> {


		if let Some(url) = url{

			let date = match creation_date {
				Some(unix_date) => unix_date,
				//TODO: Log that torrents come without creation dates
				None => utils::get_unix_time()
			};


			let current_epoch = utils::get_unix_time();
			let next_ann = current_epoch + (30*60);

			//TODO: Fix this mess

			// announce_url calculation
			let url_ = url_encoding::AnnounceUrl::new(hash.to_string(), hash.to_string());
			let announce_url = url_.serialize(&url).parse()?;


			// scrape url calc
			let url_struct = url_encoding::ScrapeUrl::new(&hash);
			let scrape_url = url_struct.announce_to_scrape(&url)?.parse()?;
			
			Ok(AnnounceComponents {
					url: Arc::new(url),
					info_hash: Arc::new(hash), 
					creation_date: Arc::new(date),
					title: Arc::new(title),
					scrape_url: scrape_url,
					announce_url: announce_url,
					client: utils::https_connection(4),
					scrape_error_count: 0,
					announce_error_count: 0,
					incomplete_data: 0,
					next_announce: next_ann,
					struct_initialization_time: current_epoch
				})
		}
		else{
			Err(Error::Torrent(TorrentErrors::NoAnnounceUrl(hash.to_string())))
		}
	}

	pub fn get(
		mut self,
		tx_announce: mpsc::Sender<AnnounceComponents>,
		tx_database: mpsc::Sender<GenericData>
		) -> () {

		let mut self_clone = self.clone();
		let tx_announce_clone = tx_announce.clone();

		let next_epoch_announce = self.allow_announce();

		// an announce is ready to fire off
		if next_epoch_announce < 0{
			self.run_announce(0, tx_announce, tx_database)
		}
		// too many scrape erros for how long the annoucer has existed
		else if self.scrape_errors_too_high() {
			self.run_announce(next_epoch_announce, tx_announce, tx_database)
		}
		// if there are too many errors _everywhere_ we kill the struct
		// and send off an error updater to where its supposed to go
		else if false {

		}
		// run a delayed announce
		else {
			self.run_scrape(tx_announce, tx_database)
		}
		// else {
		// 	self.run_
		// }


	}


	/* 
		STARTER METHOD FOR announces
	*/
	fn run_announce(mut self, delay: i64, tx_announce:mpsc::Sender<AnnounceComponents>, tx_database: mpsc::Sender<GenericData>) {
		let mut self_clone = self.clone();
		let tx_announce_clone = tx_announce.clone();

		println!{"STARTING ANNOUNCE"}

		let fut = 
		Timeout::new(self.announce() , Duration::from_secs(10))
				.map(|(data, new_interval)| {
					println!{"good announce result"}

					self.next_announce = new_interval + utils::get_unix_time();

					if self.allow_future_scrapes(&data.complete) {
						tx_announce.send(self).wait();
					}
					else{
						println!{"dropped item"}
					}

					tx_database.send(data).wait();

					})
				.map_err(|error| {
					println!{"general announce errors: {:?}", error}

					match error.into_inner(){
						Some(error) => 
							match error {
								Error::HTTP(val) => 
									match val {
										HTTPErrors::InvalidData => {		// this is likely caused by the announce being triggered to o quickly
											// self_clone.incomplete_data += 1;
											self_clone.next_announce = utils::get_unix_time() + (30 * 60);

											println!{"incomplete data <announce>, total {}", &self_clone.incomplete_data}
											}
										HTTPErrors::ParseError =>{
											self_clone.announce_error_count += 1;
											println!{"announce parse error : {:?}\nurl:\t{:?}\nhash:\t{:?}\ntitle:\t{:?}\ttotal errors {}", &val, self_clone.scrape_url, self_clone.info_hash, self_clone.title, self_clone.announce_error_count}

										}
										_=> println!{"connectin error"}
									}
								_ => println!{"timeout error announce (prob. serialize data) \nurl:\t{:?}\nhash:\t{:?}\ntitle:\t{:?}", self_clone.scrape_url, self_clone.info_hash, self_clone.title}
							}
						None => ()
					}

					tx_announce_clone.send(self_clone).wait();

				});




		let delay = utils::create_delay(delay);

		let delay_fut =
			delay
				.map(|_|{ tokio::spawn(fut);  })
				.map_err(|x| println!{"\n\n\n\n\n\n delay error this should not happen \n\n\n\n\n"});
			tokio::spawn(delay_fut);


	}

	/* 
		STARTER METHOD FOR SCRAPES
	*/
	fn run_scrape(mut self, tx_announce:mpsc::Sender<AnnounceComponents>, tx_database: mpsc::Sender<GenericData>) {

		// println!{"STARTING SCRAPE"}
		let mut self_clone = self.clone();
		let tx_announce_clone = tx_announce.clone();

		let fut = 
		Timeout::new(self.scrape() , Duration::from_secs(10))
				.map(|x| {
					if self.allow_future_scrapes(&x.complete) {
						tx_announce.send(self).wait();
					}
					else{
						println!{"dropped item"}
					}
					tx_database.send(x).wait();

					})
				.map_err(|error| {

					match error.into_inner(){
						Some(error) => 
							match error {
								Error::HTTP(val) => 
									match val {
										HTTPErrors::InvalidData => {
											self_clone.incomplete_data += 1;
											println!{"invalid scrape data. incomplete data (both) total: {}", &self_clone.incomplete_data}
											}
										HTTPErrors::ParseError =>{
											self_clone.scrape_error_count += 1;
											println!{"scrape parse error : {:?}\nurl:\t{:?}\nhash:\t{:?}\ntitle:\t{:?}\ttotal errors {}", &val, self_clone.scrape_url, self_clone.info_hash, self_clone.title, self_clone.scrape_error_count}

										}
										_=> println!{"connectin error"}
									}
								_ => println!{"timeout error (prob. serialize data) \nurl:\t{:?}\nhash:\t{:?}\ntitle:\t{:?}", self_clone.scrape_url, self_clone.info_hash, self_clone.title}
							}
						None => ()
					}

					tx_announce_clone.send(self_clone).wait();

				});

		tokio::spawn(fut);
	}

	/*

		Async code for running a scrape

	*/
	fn scrape(self: &Self) -> impl Future<Item=GenericData, Error=Error> {
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
								Err(Error::HTTP(
									HTTPErrors::InvalidData
								))
							}
						}

					},
					Err(e) => {
						Err(Error::HTTP(
							HTTPErrors::ParseError
						))
					}
				}
			});

		request
	}


	/*

		Async code for running an announce

	*/
	fn announce(self: &Self) -> impl Future<Item=(GenericData, i64), Error=Error> {
		let hash = self.info_hash.clone();
		let url = self.url.clone();
		let creation_date = self.creation_date.clone();
		let title = self.title.clone();

		let request = self.client
			// Fetch the url...
			.get(self.announce_url.clone())
			// And then, if we get a response back...
			.and_then(|res| {
				// asynchronously concatenate chunks of the body
				res.into_body().concat2()
			})
			.from_err::<Error>()
			.and_then(move |body| {
				// dbg!{"getting data"};
				let data = body.into_bytes().into_iter().collect::<Vec<_>>();

				match AnnounceData::new_bytes(&data) {
					Ok(announce) => {
						
						let new_interval = 
							if let Some(new_interval) = announce.interval{new_interval}
							else{600};


						let gen_data = 
							GenericData::new(
								hash,
								url,
								creation_date,
								title,
								announce.downloaded,
								announce.complete,
								announce.incomplete);

						Ok((gen_data, new_interval))

					},
					Err(e) => {
						Err(Error::HTTP(
							HTTPErrors::InvalidData
						))
					}
				}
			});

		request
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
	
	fn allow_announce(&self) -> i64{
		let now = utils::get_unix_time();
		// let diff = now - self.next_announce;
		let diff = self.next_announce - now;


		diff
	}

	fn scrape_errors_too_high(&self) -> bool{
		let now = utils::get_unix_time();
		let hours = (now - self.struct_initialization_time) / 3600;
		let hours = 
			if hours == 0 {1}
			else {hours};


		if (self.scrape_error_count / hours) >= 5 {true}
		else {false}
	}
	
}