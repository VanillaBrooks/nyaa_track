mod read_torrent;
use read_torrent::Torrent;
// mod serde_test;
mod requests;

#[macro_use]
extern crate lazy_static;

use requests::rss_parse::{self, get_xml};
use requests::url_encoding::{Url, hex_to_char};
// use hashbrown::HashMap;
use std::collections::HashMap;

fn create_torrent(path: &str) ->() {
	let k = Torrent::new_file(path);

	match k{
		Ok(x) => {
			println!{"all good"}
			dbg!{x};
		} 
		Err(x) => println!{"there was an error:\n{:?}",x}
	}
}

use serde_urlencoded::ser;
fn main() {
	// // nyaa rss feed
	// let nyaa_rss_feed = "https://nyaa.si/?page=rss";
	
	
	// // reading stored torrent files
	// let mut torrent_files = r"C:\Users\Brooks\Downloads\torrent files\".to_string();
	// let mut downloads = r"C:\Users\Brooks\Downloads\".to_string();
	// let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();

	// git_torrents.push_str("1145877.torrent");
	// let file = &git_torrents;
	// dbg!{Torrent::new_file(file)};
	
	// downloading torrents from internet
	// let torrent_url = r"https://nyaa.si/download/1145877.torrent";

	// let res = rss_parse::download_torrent(Some(torrent_url));
	// println!{"nyaa response: "}
	// dbg!{res};

	// hex_to_char();

	// 1.to_string().as_str()

	// get_xml("Test");

	// let url_struct = Url::new("026a8f0bc3194dbbe545ffd2409ea9cc1f6b7776".to_string(), "026a8f0bc3194dbbe545ffd2409ea9cc1f6b7776".to_string());
	// let k = serde_urlencoded::to_string(url_struct);
	// dbg!{k};

	let to_convert = "123456789abcdef123456789abcdef123456789a";

	let hm = hex_to_char(to_convert);
	dbg!{&hm};

	assert_eq!("%124Vx%9a%bc%de%f1%23Eg%89%ab%cd%ef%124Vx%9a".to_ascii_uppercase(), hm.to_uppercase())



}
