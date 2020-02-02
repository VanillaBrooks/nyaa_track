// use reqwest;
use futures::channel::mpsc;

use std::sync::Arc;

use super::super::read::AnnounceComponents;
use rss;
use std::fs;
use std::io::prelude::*;
use std::thread;
use std::time::{self, Duration};

use std::collections::HashSet;

use super::super::utils;

use super::super::error::*;

use parking_lot::RwLock;

macro_rules! parse {
    ($parse_funct:ident, $parse_item:ident, $previous:ident, $dl:ident, $tx:ident) => {
        match $parse_funct(&$parse_item) {
            Ok(info_hash) => {
                // make sure we are not grabbing an old hash
                {
                    let previous = $previous.read();
                    if previous.contains(info_hash) {
                        continue;
                    } else {
                        println! {"info hash {:?} was not in previous", info_hash}
                    }
                }
                // if we are about to download the torrent sleep it for 1 second
                // prevents tracker from blocking us
                thread::sleep(Duration::from_millis(300));

                // make sure the link is good
                match $parse_item.link() {
                    Some(good_url) => {
                        let tx = $tx.clone();

                        let timeout_fut = tokio::time::timeout(
                            Duration::from_secs(5),
                            $dl.download(good_url, info_hash.to_string(), tx),
                        )
                        .await;

                        if timeout_fut.is_ok() {
                            println! {"recieved good torrent data"}
                        } else {
                            println! {"Error downloading torrent data"}
                        }
                    }

                    None => println! {"error with link"},
                }
            }
            Err(_error) => {
                println! {"error with RSS item"};
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
pub async fn get_xml<T: Send + Sync + std::hash::BuildHasher + 'static>(
    url: &str,
    previous: Arc<RwLock<HashSet<String, T>>>,
    tx_to_filter: mpsc::Sender<AnnounceComponents>,
) -> Result<(), Error> {
    // decide what hash-parsing function we will use for the given url
    let parse_funct = if url.contains(".si") {
        nyaa_si_hash
    } else {
        nyaa_pantsu_hash
    };

    let client = utils::https_connection();
    let uri = url.parse().expect("rss url invalid");

    let res = client.get(uri).await?.into_body();
    let res_bytes = hyper::body::to_bytes(res)
        .await?
        .into_iter()
        .collect::<Vec<u8>>();

    let mut path: String = r".\temp".into();

    // create the temp directory and ignore any error
    match std::fs::create_dir(&path) {
        _ => (),
    };

    path.push_str(r"\");
    path.push_str(&utils::get_unix_time().to_string());
    path.push_str(".xml");

    let mut file = fs::File::create(&path).expect("xml file could not be created");
    if let Err(err) = file.write_all(&res_bytes) {
        println! {"error writing rss feed to file:"}
        dbg! {err};
    }

    let channel = rss::Channel::read_from(res_bytes.as_slice()).expect("error when reading rss");
    let mut items = channel.into_items().to_vec();

    // std::fs::remove_file(path);

    let dl = utils::Downloader::default();

    for _ in 0..items.len() {
        let item = items.remove(0);

        parse! {parse_funct, item, previous, dl, tx_to_filter};
    }

    Ok(())
}

// timer for rss updates
pub struct Timer<'a> {
    last_check: time::Instant,
    time_between: u32, // in seconds
    pub url: &'a str,
}
impl<'a> Timer<'a> {
    pub fn new(between: u32, url: &'a str) -> Timer<'a> {
        Timer {
            last_check: time::Instant::now() - Duration::from_secs(u64::from(between) + 1),
            time_between: between,
            url,
        }
    }
    pub fn allow_check(&mut self) -> bool {
        let now = time::Instant::now();
        let elapsed = (now - self.last_check).as_secs() as u32;
        if elapsed > self.time_between {
            self.last_check = now;
            true
        } else {
            false
        }
    }
}

// do this since ? does not work w/ Option<T>
fn nyaa_si_hash(item: &rss::Item) -> Result<&str, Error> {
    let ext = item.extensions();

    match ext.get("nyaa") {
        Some(nyaa) => match nyaa.get("infoHash") {
            Some(extension_vec) => {
                if extension_vec.len() == 1 {
                    let ext_index = &extension_vec[0];
                    match ext_index.value() {
                        Some(infohash) => Ok(infohash),
                        None => Err(Error::Rss(RssErrors::InfoHashFetch("No value field"))),
                    }
                } else {
                    Err(Error::Rss(RssErrors::InfoHashFetch(
                        "!= one item in the vector",
                    )))
                }
            }
            None => Err(Error::Rss(RssErrors::InfoHashFetch("no field infohash"))),
        },
        None => Err(Error::Rss(RssErrors::InfoHashFetch("No field nyaa"))),
    }
}

fn nyaa_pantsu_hash(item: &rss::Item) -> Result<&str, Error> {
    let link = item.link();
    match link {
        Some(data) => utils::content_after_last_slash(&data),
        None => Err(Error::Rss(RssErrors::CouldNotReadRss)),
    }
}
