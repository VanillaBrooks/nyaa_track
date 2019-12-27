use super::super::{database::connection, error::*, requests::url_encoding, utils};
use std::time::Duration;

use super::{AnnounceData, GenericData, ScrapeData};
use futures::channel::mpsc;

use futures::SinkExt;

use std::sync::Arc;

use hyper::client::{Client, HttpConnector};
// use hyper::rt::{Future, Stream};
use hyper_tls::HttpsConnector;

use tokio::time::Delay;

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
                scrape_url,
                announce_url,
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

    pub async fn get(
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
                .await;
        }
        // run a (potentially) delayed scrape
        else {
            let fut = async move {
                // TODO: make sure generate delay is taking in the right args here. its looks suspicious
                let delay = self.time_existance().generate_delay(1).await;
                self.run_scrape(tx_announce, tx_database).await
            };

            tokio::spawn(fut);
        }
    }

    /*
        STARTER METHOD FOR announces
    */
    async fn run_announce(
        mut self,
        delay: i64,
        mut tx_announce: mpsc::Sender<AnnounceComponents>,
        mut tx_database: mpsc::Sender<connection::DatabaseUpsert>,
    ) {

        println! {"STARTING ANNOUNCE"}

        let fut = async move {
            let timeout_fut = tokio::time::timeout(Duration::from_secs(10), self.announce()).await;

            if let Ok(res) = timeout_fut {
                match res {
                    Ok((data, new_interval)) => {
                        println! {"good announce result"}

                        self.next_announce = new_interval + utils::get_unix_time();

                        if self.allow_future_scrapes(&data.complete) {
                            tx_announce.send(self).await;
                        } else {
                            println! {"dropped item"}
                        }

                        let db_wrap = connection::DatabaseUpsert::Data(data);
                        tx_database.send(db_wrap).await;
                    }
                    Err(error) => {
                        // // TODO: clean this horrific thing up
                        // match error.into_inner(){
                        //     Some(error) =>
                        //         match error {
                        //             Error::HTTP(val) =>
                        //                 match val {
                        //                     HTTPErrors::InvalidData => {		// this is likely caused by the announce being triggered to o quickly
                        //                         println!{"announce invalid data being logged to database"}

                        //                         let now = utils::get_unix_time();
                        //                         self_clone.incomplete_data += 1;
                        //                         self_clone.next_announce = now + (30 * 60);
                        //                         let db_wrap = connection::ErrorType::new(connection::ErrorType::InvalidAnnounce, self_clone.info_hash.clone(), now);

                        //                         tx_database_clone.send(db_wrap).await;
                        //                     }
                        //                     HTTPErrors::ParseError =>{
                        //                         self_clone.announce_error_count += 1;
                        //                         println!{"announce parse error : {:?}\nurl:\t{:?}\nhash:\t{:?}\ntitle:\t{:?}\ttotal errors {}", &val, self_clone.scrape_url, self_clone.info_hash, self_clone.title, self_clone.announce_error_count}

                        //                     }
                        //                     _=> println!{"connectin error"}
                        //                 }
                        //             _ => println!{"timeout error announce (prob. serialize data) \nurl:\t{:?}\nhash:\t{:?}\ntitle:\t{:?}", self_clone.scrape_url, self_clone.info_hash, self_clone.title}
                        //         }
                        //     None => ()
                        // }

                        tx_announce.send(self).await;
                    }
                };
            } else {
                // TODO : log the error here
                tx_announce.send(self).await;
            }
        };

        let delay = utils::create_delay(delay).await;
        tokio::spawn(fut);
    }

    /*
        STARTER METHOD FOR SCRAPES
    */
    async fn run_scrape(
        mut self,
        mut tx_announce: mpsc::Sender<AnnounceComponents>,
        mut tx_database: mpsc::Sender<connection::DatabaseUpsert>,
    ) {
        // println!{"STARTING SCRAPE"}
        let fut = async move {
            let timeout_fut = tokio::time::timeout(Duration::from_secs(10), self.scrape()).await;
            // check if the timeout was ok
            if let Ok(res) = timeout_fut {
                // check the contents of the actual scrape data
                if let Ok(scrape_data) = res {
                    if self.allow_future_scrapes(&scrape_data.complete) {
                        tx_announce.send(self).await;
                    } else {
                        println! {"dropped item"}
                    }
                    let db_wrap = connection::DatabaseUpsert::Data(scrape_data);
                    tx_database.send(db_wrap).await;
                }
                // the scrape data was not ok
                else {
                    // TODO: log this error
                    tx_announce.send(self).await;
                }
            }
            // the timeout was not ok
            else {
                // TODO: log the error
                println! {"the timeout expired for a scrape"}
                tx_announce.send(self).await;
            }
        };

        tokio::spawn(fut);
    }

    /*

        Async code for running a scrape

    */
    async fn scrape(self: &Self) -> Result<GenericData, Error> {
        // get data, convert to body
        let request = self.client.get(self.scrape_url.clone()).await?.into_body();
        // convert body to bytes
        let request_bytes = hyper::body::to_bytes(request)
            .await?
            .into_iter()
            .collect::<Vec<_>>();

        // parse the data
        if let Ok(scrape) = ScrapeData::new_bytes(&request_bytes) {
            // get the first value from the data
            // TODO: better api for this
            if let Some(data) = scrape.files.values().into_iter().next() {
                // package all data into one generic struct
                let generic_data = GenericData::new(
                    self.info_hash.clone(),
                    self.url.clone(),
                    self.creation_date.clone(),
                    self.title.clone(),
                    data.downloaded,
                    data.complete,
                    data.incomplete,
                );
                Ok(generic_data)
            } else {
                Err(Error::HTTP(HTTPErrors::InvalidData))
            }
        } else {
            Err(Error::HTTP(HTTPErrors::InvalidData))
        }
    }

    /*

        Async code for running an announce

    */
    async fn announce(self: &Self) -> Result<(GenericData, i64), Error> {
        let request = self
            .client
            .get(self.announce_url.clone())
            .await?
            .into_body();
        let request_bytes = hyper::body::to_bytes(request)
            .await?
            .into_iter()
            .collect::<Vec<u8>>();

        if let Ok(announce) = AnnounceData::new_bytes(&request_bytes) {
            // fetch the tracker timeout before the next announce is allowed
            let new_interval = if let Some(interval) = announce.interval {
                interval
            } else {
                600
            };

            // store data generically
            let gen_data = GenericData::new(
                self.info_hash.clone(),
                self.url.clone(),
                self.creation_date.clone(),
                self.title.clone(),
                announce.downloaded,
                announce.complete,
                announce.incomplete,
            );
            Ok((gen_data, new_interval))
        } else {
            Err(Error::HTTP(HTTPErrors::InvalidData))
        }
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
        let days = hours / 24;

        seconds -= hours * 3600;

        hours -= days * 24;

        Self {
            days,
            hours,
            seconds,
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
