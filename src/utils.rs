use futures::{Future, Stream};
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use std::io::prelude::*;

use futures::channel::mpsc;
use futures::sink::Sink;
use futures::SinkExt;

// mod error;
use super::error::*;

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tokio;

use hashbrown::HashSet;

use super::read::{announce_components, torrent};
use announce_components::AnnounceComponents;
use torrent::Torrent;
// use announce_result::AnnounceResult;

pub fn https_connection(thread_count: usize) -> Client<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    return client;
}

// TODO: configure client pooling
// probably want to turn this thing into a struct
pub struct Downloader {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Downloader {
    pub fn new() -> Downloader {
        let client_config = https_connection(4);
        Downloader {
            client: client_config,
        }
    }
    pub fn from_client(client: Client<HttpsConnector<HttpConnector>>) -> Self {
        Downloader { client: client }
    }
    pub async fn download(
        &self,
        url: &str,
        hash: String,
        mut tx: mpsc::Sender<AnnounceComponents>,
    ) -> Result<(), Error> {
        let url = url
            .parse()
            .expect("URI was not able to be parsed correctly in Downloader::download");

        let res = self.client.get(url).await?.into_body();
        let res_bytes = hyper::body::to_bytes(res)
            .await?
            .into_iter()
            .collect::<Vec<u8>>();

        match Torrent::new_bytes(&res_bytes) {
            Ok(torrent) => {
                let ann = torrent_to_announce_components(torrent, &hash)?;
                tx.send(ann).await?;
                Ok(())
            }
            Err(e) => Err(e),
        }

    }
}

// generate a .torrent file for the data
pub fn write_torrent_to_file(data: &Vec<u8>, save_name: &str) -> Result<String, Error> {
    //TODO: async file write here
    let mut base = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
    base.push_str(save_name);
    base.push_str(".torrent");

    let mut file = std::fs::File::create(&base)?;
    file.write_all(&data)?;

    return Ok(base);
}

// BASE SAVE PATH
pub fn content_after_last_slash<'a>(url: &'a str) -> Result<&'a str, Error> {
    let mut last = 0;
    for i in 0..url.len() - 1 {
        let k = match url.get(i..i + 1) {
            Some(data) => data,
            None => return Err(Error::SliceError("Could not slice the string".to_string())),
        };
        if k == "/" || k == r"\" {
            last = i;
        }
    }

    match url.get(last + 1..url.len()) {
        Some(slice) => Ok(slice),
        None => Err(Error::SliceError(
            "did not contain a slash. you fucked up somewhere".to_string(),
        )),
    }
}

pub fn content_before_last_slash<'a>(url: &'a str) -> Result<&'a str, Error> {
    let mut last = 0;

    for i in 0..url.len() - 1 {
        let k = match url.get(i..i + 1) {
            Some(data) => data,
            None => return Err(Error::SliceError("Could not slice the string".to_string())),
        };
        if k == "/" || k == r"\" {
            last = i;
        }
    }

    match url.get(0..last + 1) {
        Some(slice) => Ok(slice),
        None => Err(Error::SliceError(
            "did not contain a slash. you fucked up somewhere".to_string(),
        )),
    }
}

// asssumes it is only filename and .torrent with no extra directory info
pub fn content_before_dot_torrent<'a>(input: &'a str) -> Result<&'a str, Error> {
    match input.find(".") {
        Some(index) => match input.get(0..index) {
            Some(x) => Ok(x),
            None => Err(Error::SliceError("indexes of slice invalud".to_string())),
        },
        None => Err(Error::SliceError("could not slice .torrent".to_string())),
    }
}

pub fn compare_files(f1: &str, f2: &str) -> Result<(), Error> {
    let mut buffer1 = Vec::new();
    let mut file1 = std::fs::File::open(f1).unwrap();
    file1.read_to_end(&mut buffer1)?;

    let mut buffer2 = Vec::new();
    let mut file2 = std::fs::File::open(f2).unwrap();
    file2.read_to_end(&mut buffer2)?;

    println! {"f1 len:\t{}\tf2 len:\t{}",buffer1.len(), buffer2.len()}

    let len; // might be the source of a bug here
    if buffer1.len() > f2.len() {
        len = buffer2.len()
    } else {
        len = buffer1.len()
    }

    for i in 0..(len - 1) {
        let c1 = &buffer1[i];
        let c2 = &buffer2[i];
        if c1 == c2 {
            println! {"match"}
            continue;
        } else {
            println! {"{} {} {}", i, buffer1[i], buffer2[i]}
            break;
        }
    } //for

    Ok(())
}

pub fn get_unix_time() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
}

// gives all torrents in directory (good and bad) and the path to them
fn serialize_all_torrents(directory: &str) -> Vec<(String, Result<Torrent, Error>)> {
    let dir: Vec<_> = std::fs::read_dir(directory)
        .unwrap()
        .map(|x| x.unwrap().path())
        .map(|x| {
            let text_path = x.to_str().unwrap();
            let torrent = Torrent::new_file(&text_path);
            (text_path.to_string(), torrent)
        })
        .collect();

    return dir;
}

// returns ONLY GOOD torrents with their info hashes manually inserted from tracker
pub fn torrents_with_hashes(directory: &str) -> Vec<Torrent> {
    let torrents = serialize_all_torrents(directory);
    let mut results = Vec::with_capacity(torrents.len());

    torrents
        .into_iter()
        .filter(|(_, y)| y.is_ok())
        .for_each(|(x, y)| {
            let a = content_after_last_slash(&x).unwrap();
            let b = content_before_dot_torrent(&a).unwrap();

            match y {
                Ok(mut torrent) => {
                    torrent.set_info_hash(b);
                    results.push(torrent);
                }
                Err(_) => (),
            }
        });

    results
}

//TODO: compose this function with `torrents_with_hashes`
// filter all torrents to only be nyaa.si announce URLS
pub fn nyaa_si_announces_from_files(directory: &str) -> Vec<AnnounceComponents> {
    let all_torrents = torrents_with_hashes(directory);
    all_torrents
        .into_iter()
        .filter(|x| {
            // make sure it has the url we are looking for
            match &x.announce {
                Some(ann_url) => {
                    if ann_url.contains("http") && ann_url.contains("nyaa") {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            }
        })
        .map(|mut x| {
            let k = x.info_hash();
            AnnounceComponents::new(
                x.announce,
                k.unwrap(),
                x.creation_date,
                x.info.name().unwrap(),
            )
        })
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
}

pub fn filter_nyaa_announces(data: Vec<AnnounceComponents>) -> Vec<AnnounceComponents> {
    data.into_iter()
        .filter(|x| {
            // make sure it has the url we are looking for

            if x.url.contains("http") && x.url.contains("nyaa") {
                true
            } else {
                false
            }
        })
        .collect::<Vec<_>>()
}

pub fn info_hash_set(directory: &str) -> HashSet<String> {
    let mut hash_set: HashSet<String> = HashSet::new();

    torrents_with_hashes(directory)
        .into_iter()
        .for_each(|mut x| {
            hash_set.insert(x.info_hash().unwrap());
        });

    return hash_set;
}

pub fn check_hashes(dir_to_read: &str) -> () {
    //Vec<(String, Torrent)>{

    let dir: Vec<_> = serialize_all_torrents(dir_to_read);

    let mut good = 0;
    let mut bad: Vec<String> = Vec::new();

    for (filename, torrent) in dir {
        // println!{"handling: {}", filename}

        match torrent {
            Ok(mut torrent) => {
                let hash = content_before_dot_torrent(&filename).unwrap();
                let hash = content_after_last_slash(&hash).unwrap();

                if hash == torrent.info_hash().unwrap() {
                    good += 1;
                } else {
                    println! {"{}\n{}\n do not match \n\n", hash, torrent.info_hash().unwrap()}
                    bad.push(hash.to_string());
                }
            }
            Err(err) => println! {"Error parsing torrent {} : {:?}", filename, err},
        }
    }

    println! {"good hashes:\t {}\tbad hashes:\t {}", good, bad.len()}
    if bad.len() > 0 {
        dbg! {bad};
    }
}

pub fn torrent_to_announce_components(
    torrent: Torrent,
    info_hash: &str,
) -> Result<AnnounceComponents, Error> {
    match torrent.info.name() {
        Ok(name) => {
            let ann = AnnounceComponents::new(
                torrent.announce,
                info_hash.to_string(),
                torrent.creation_date,
                name,
            )?;
            Ok(ann)
        }
        Err(_) => Err(Error::Torrent(TorrentErrors::MissingName)),
    }
}

pub fn create_delay(seconds: i64) -> tokio::time::Delay {
    let now = tokio::time::Instant::now();
    let fut_time = now
        .checked_add(tokio::time::Duration::new(seconds as u64, 0))
        .expect("DELAY ERROR: not in the future");

    // Delay::new(fut_time)
    tokio::time::delay_until(fut_time)
}
