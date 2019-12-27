use super::super::{database::connection, error::*, requests::url_encoding, utils};

use std::time::Duration;

use super::{AnnounceData, GenericData, ScrapeData};
use futures::future;
use futures::sync::mpsc;
use futures::Sink;

use std::sync::Arc;

use hyper::client::{Client, HttpConnector};
use hyper::rt::{Future, Stream};
use hyper_tls::HttpsConnector;

use tokio::timer::{Delay, Timeout};

enum RequestType {
    Announce((GenericData, i64)),
    Scrape(GenericData),
}

#[derive(Debug, Clone)]
pub struct AnnounceComponents {
    pub url: Arc<String>,
    pub info_hash: Arc<String>,
    pub title: Arc<String>,
    pub creation_date: i64,
    scrape_url: hyper::Uri,
    announce_url: hyper::Uri,
    client: Client<HttpsConnector<HttpConnector>>,
    scrape_error_count: i64,
    announce_error_count: i64,
    incomplete_data: i64,
    next_announce: i64,
    struct_initialization_time: i64,
}

const SCRAPE_URL: &str = "http://nyaa.tracker.wf:7777/announce";

// TODO: fix unwrap
impl<'a> AnnounceComponents {
    pub fn new(
        url: Option<String>,
        hash: String,
        creation_date: Option<i64>,
        title: String,
    ) -> Result<AnnounceComponents, Error> {
        if let Some(url) = url {
            let date = match creation_date {
                Some(unix_date) => unix_date,
                //TODO: Log that torrents come without creation dates
                None => utils::get_unix_time(),
            };

            let current_epoch = utils::get_unix_time();
            let next_ann = current_epoch + (30 * 60);

            //TODO: Fix this mess

            // announce_url calculation
            let url_ = url_encoding::AnnounceUrl::new(hash.to_string(), hash.to_string());
            let announce_url = url_.serialize(SCRAPE_URL).parse()?;

            // scrape url calc
            let url_struct = url_encoding::ScrapeUrl::new(&hash);
            let scrape_url = url_struct.announce_to_scrape(SCRAPE_URL)?.parse()?;

            Ok(AnnounceComponents {
                url: Arc::new(url),
                info_hash: Arc::new(hash),
                creation_date: date,
                title: Arc::new(title),
                scrape_url: scrape_url,
                announce_url: announce_url,
                client: utils::https_connection(4),
                scrape_error_count: 0,
                announce_error_count: 0,
                incomplete_data: 0,
                next_announce: next_ann,
                struct_initialization_time: current_epoch,
            })
        } else {
            Err(Error::Torrent(TorrentErrors::NoAnnounceUrl(
                hash.to_string(),
            )))
        }
    }

    pub fn get(
        self,
        tx_announce: mpsc::Sender<AnnounceComponents>,
        tx_database: mpsc::Sender<connection::DatabaseUpsert>,
    ) -> () {
        //

        let next_epoch_announce = self.allow_announce();

        if self.scrape_errors_too_high() && self.announce_errors_too_high() {
            () // kill the struct
        }
        // too many scrape erros for how long the annoucer has existed
        else if self.scrape_errors_too_high() {
            self.run_announce(next_epoch_announce, tx_announce, tx_database)
        }
        // run a (potentially) delayed scrape
        else {
            let delay = self.time_existance().generate_delay(1);

            let del_fut = 
				delay.map(|_| self.run_scrape(tx_announce, tx_database) )
				.map_err(|_| println!{"\n\n\n\n erorr spawning scrape future delay this should not happen \n\n\n\n"});
            tokio::spawn(del_fut);
        }
    }

    /*
        STARTER METHOD FOR announces
    */
    fn run_announce(
        mut self,
        delay: i64,
        tx_announce: mpsc::Sender<AnnounceComponents>,
        tx_database: mpsc::Sender<connection::DatabaseUpsert>,
    ) {
        let mut self_clone = self.clone();
        let tx_announce_clone = tx_announce.clone();
        let tx_database_clone = tx_database.clone();

        println! {"STARTING ANNOUNCE"}

        let fut = 
		Timeout::new(self.announce() , Duration::from_secs(10))
				.map(|(data, new_interval)| {
					println!{"good announce result"}

					self.next_announce = new_interval + utils::get_unix_time();

					if self.allow_future_scrapes(&data.complete) {
						tx_announce.send(self).wait();
					} else{
						println!{"dropped item"}
					}

					
					let db_wrap = connection::DatabaseUpsert::Data(data);
					tx_database.send(db_wrap).wait();

					})
				.map_err(|error| {
					// println!{"general announce errors: {:?}\n\tannounce url: {}\n\tscrape_url: {}", error, self_clone.announce_url, self_clone.scrape_url}

					match error.into_inner(){
						Some(error) => 
							match error {
								Error::HTTP(val) => 
									match val {
										HTTPErrors::InvalidData => {		// this is likely caused by the announce being triggered to o quickly
											println!{"announce invalid data being logged to database"}

											let now = utils::get_unix_time();
											self_clone.incomplete_data += 1;
											self_clone.next_announce = now + (30 * 60);
											let db_wrap = connection::ErrorType::new(connection::ErrorType::InvalidAnnounce, self_clone.info_hash.clone(), now);
										
											tx_database_clone.send(db_wrap).wait();
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

        let delay_fut = delay
            .map(|_| {
                tokio::spawn(fut);
            })
            .map_err(|x| println! {"\n\n\n\n\n\n delay error this should not happen \n\n\n\n\n"});
        tokio::spawn(delay_fut);
    }

    /*
        STARTER METHOD FOR SCRAPES
    */
    fn run_scrape(
        mut self,
        tx_announce: mpsc::Sender<AnnounceComponents>,
        tx_database: mpsc::Sender<connection::DatabaseUpsert>,
    ) {
        // println!{"STARTING SCRAPE"}
        let mut self_clone = self.clone();
        let tx_announce_clone = tx_announce.clone();
        let tx_database_clone = tx_database.clone();

        let fut = 
		Timeout::new(self.scrape() , Duration::from_secs(10))
				.map(|x| {
					if self.allow_future_scrapes(&x.complete) {
						tx_announce.send(self).wait();
					}
					else{
						println!{"dropped item"}
					}
					let db_wrap = connection::DatabaseUpsert::Data(x);
					tx_database.send(db_wrap).wait();

					})
				.map_err(|error| {

					// println!{"general scrape errors: {:?}\nnnounce url: {}\n\tscrape_url: {}", error, self_clone.announce_url, self_clone.scrape_url}
					match error.into_inner(){
						Some(error) => 
							match error {
								Error::HTTP(val) => 
									match val {
										HTTPErrors::InvalidData => {
											println!{"scrape invalid data being logged to database"}

											self_clone.incomplete_data += 1;

											let now = utils::get_unix_time();
											let db_wrap = connection::ErrorType::new(connection::ErrorType::InvalidAnnounce, self_clone.info_hash.clone(), now);
											tx_database_clone.send(db_wrap).wait();

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
    fn scrape(self: &Self) -> impl Future<Item = GenericData, Error = Error> {
        let hash = self.info_hash.clone();
        let hash_clone = self.info_hash.clone();
        let url = self.url.clone();
        let creation_date = self.creation_date.clone();
        let title = self.title.clone();

        let request = self
            .client
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
                        match scrape.files.values().into_iter().next() {
                            Some(data) => {
                                let gen_data = GenericData::new(
                                    hash,
                                    url,
                                    creation_date,
                                    title,
                                    data.downloaded,
                                    data.complete,
                                    data.incomplete,
                                );

                                Ok(gen_data)
                            }
                            None => Err(Error::HTTP(HTTPErrors::InvalidData)),
                        }
                    }
                    Err(e) => Err(Error::HTTP(HTTPErrors::ParseError)),
                }
            });

        request
    }

    /*

        Async code for running an announce

    */
    fn announce(self: &Self) -> impl Future<Item = (GenericData, i64), Error = Error> {
        let hash = self.info_hash.clone();
        let url = self.url.clone();
        let creation_date = self.creation_date.clone();
        let title = self.title.clone();

        let request = self
            .client
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
                        let new_interval = if let Some(new_interval) = announce.interval {
                            new_interval
                        } else {
                            600
                        };

                        let gen_data = GenericData::new(
                            hash,
                            url,
                            creation_date,
                            title,
                            announce.downloaded,
                            announce.complete,
                            announce.incomplete,
                        );

                        Ok((gen_data, new_interval))
                    }
                    Err(e) => Err(Error::HTTP(HTTPErrors::InvalidData)),
                }
            });

        request
    }

    fn allow_future_scrapes(&self, seeders: &i64) -> bool {
        let days_alive = (utils::get_unix_time() - self.creation_date) / 86400;

        // older than 7 days, less than 100 active seeders we terminate tracking
        if days_alive > 7 && *seeders < 100 {
            false
        } else {
            true
        }
    }

    fn allow_announce(&self) -> i64 {
        let now = utils::get_unix_time();
        // let diff = now - self.next_announce;
        let diff = self.next_announce - now;

        diff
    }

    fn scrape_errors_too_high(&self) -> bool {
        let now = utils::get_unix_time();
        let hours = (now - self.struct_initialization_time) / 3600;
        let hours = if hours == 0 { 1 } else { hours };

        if (self.scrape_error_count / hours) >= 5 {
            true
        } else {
            false
        }
    }

    fn announce_errors_too_high(&self) -> bool {
        let now = utils::get_unix_time();
        let hours = (now - self.struct_initialization_time) / 3600;
        let hours = if hours == 0 { 1 } else { hours };

        if (self.announce_error_count / hours) > 2 && self.announce_error_count > 10 {
            true
        } else {
            false
        }
    }

    // Time since the creation of the torrent file *NOT* when the struct was created
    fn time_existance(&self) -> ElapsedTime {
        let now = utils::get_unix_time();

        let diff = now - self.creation_date;

        ElapsedTime::new(diff)
    }
}

#[derive(Debug, Clone)]
pub struct ElapsedTime {
    pub days: i64,
    pub hours: i64,
    pub seconds: i64,
}
impl ElapsedTime {
    pub fn new(mut seconds: i64) -> Self {
        let mut hours = seconds / 3600;
        let mut days = hours / 24;

        seconds -= hours * 3600;

        hours -= days * 24;

        Self {
            days: days,
            hours: hours,
            seconds: seconds,
        }
    }

    /// Returns number of seconds to delay the scrape
    pub fn generate_delay(&self, min_days: i64) -> Delay {
        let days_delay = 3;

        let delay = if self.days < min_days {
            0
        } else {
            // delay by "days existed" as minutes TIMES delay constant
            self.days * 60 * days_delay
        };
        utils::create_delay(delay)
    }
}
