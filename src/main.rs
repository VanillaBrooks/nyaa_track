mod read_torrent;
use read_torrent::Torrent;

pub mod requests;
pub mod error;
pub mod utils;

#[macro_use]
extern crate lazy_static;

use requests::rss_parse::{self, get_xml, AnnounceComponents};
use requests::url_encoding::{Url, hex_to_char};
// use hashbrown::HashMap;


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

use std::io::prelude::*;
use std::fs::File;

use bencode::Bencode;
use bencode::ToBencode;
use serde_urlencoded::ser;
fn main() {
	// // nyaa rss feed
	let nyaa_rss_feed = "https://nyaa.si/?page=rss";
	
	
	// // reading stored torrent files
	let mut torrent_files = r"C:\Users\Brooks\Downloads\torrent files\".to_string();
	let mut downloads = r"C:\Users\Brooks\Downloads\".to_string();
	let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
	
	// let mut k = get_xml(nyaa_rss_feed).unwrap();
	// requests::tracking::announce_components(&mut k.good);
	// let mut cpy = r"C:\Users\Brooks\github\nyaa_tracker\torrents\test.txt";
	// git_torrents.push_str("1147506.torrent");

	// let mut writer =  File::create(cpy).unwrap();

	// let mut torrent = read_torrent::Torrent::new_file(&git_torrents);
	// dbg!{torrent};
	// let bc = torrent.to_bencode().to_bytes().unwrap();
	// writer.write(&bc);
	// bc.to_writer(&mut writer);
	// dbg!{k};

	// let dir = r"C:\Users\Brooks\github\nyaa_tracker\torrents";
	// let torrents = utils::serialize_all_torrents(dir);
	// dbg!{torrents.len()};
	// dbg!{&torrents[0]};


	let loc = r"C:\Users\Brooks\Downloads\test.txt";

	let torrent = read_torrent::TestInfo::new(&loc);

	dbg!{&torrent};


}

