mod read;
use read::{announce_components, announce_result, torrent};
use announce_components::AnnounceComponents;
use announce_result::AnnounceResult;
use torrent::Torrent;

pub mod requests;
pub mod error;
pub mod utils;
pub mod database;


use error::Error;

#[macro_use]
extern crate lazy_static;

use requests::rss_parse::{self, get_xml};
use requests::url_encoding::{Url, hex_to_char};
use hashbrown::HashMap;

use std::io::prelude::*;
use std::fs::File;

use bencode::Bencode;
use bencode::ToBencode;
use serde_urlencoded::ser;

use hashbrown::HashSet;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn write_torrent(read: &str, write: &str) -> Result<(), Error> {
    let torrent = Torrent::new_file(&read)?;
	let x = torrent.to_bencode().to_bytes()?;
	let mut file = File::create(&write)?;

	file.write(&x)?;

	Ok(())

}

#[allow(dead_code)]
fn read_data (loc: &str ) -> Result<(), Error> {
	let mut buffer = Vec::new();

	let mut file = File::open(&loc)?;


	file.read_to_end(&mut buffer)?;
	let ans = torrent::sha1(&buffer);

	dbg!{ans};
	Ok(())
}

#[allow(dead_code)]
fn load_problem_hash(hash: &str)  {
	let mut file = TORRENTS_DIR.clone().to_string();
	file.push_str(&hash);
	file.push_str(".torrent");
	let torrent = Torrent::new_file(&file);

	match torrent {
		Ok(x)=> {
			// let i = x.info;
			dbg!{x};
		},
		Err(x)=> println!{"error loading hash {} : {:?}", hash, x}
	}
}

const TORRENTS_DIR : &str= r"C:\Users\Brooks\github\nyaa_tracker\torrents\";
const SI_RSS: &str = r"https://nyaa.si/?page=rss";
#[allow(dead_code)]
const PANTSU_RSS : &str = r"https://nyaa.pantsu.cat/feed?";
#[allow(dead_code)]
const TEST_FILE :&str=  r"C:\Users\Brooks\Downloads\test.txt";

fn main() {

	// let mut previous = utils::info_hash_set(TORRENTS_DIR);
	// // let mut previous = HashSet::new();
	// println!{"starting si "};
	// let ans = get_xml(&SI_RSS, &mut previous);
	// println!{"starting pantsu"};
	// let ans =get_xml(&PANTSU_RSS, &mut previous);

	// let mut announces = utils::nyaa_si_announces(TORRENTS_DIR);

	// for i in 1..announces.len(){
	// 	dbg!{&announces[i].info_hash};
	// 	dbg!{&announces[i].announce().unwrap()};
	// }


	// let read = "C:\\Users\\Brooks\\github\\nyaa_tracker\\torrents\\f8e64792b00bee14249ba5cf18fb6d7b71b0fc58.torrent";
	// write_torrent(&read, &test_file);
	// load_problem_hash("f8e64792b00bee14249ba5cf18fb6d7b71b0fc58");
	// utils::check_hashes(&torrents_dir);
}

