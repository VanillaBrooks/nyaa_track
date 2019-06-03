mod read_torrent;
use read_torrent::Torrent;

pub mod requests;
pub mod error;
pub mod utils;
pub mod database;

// use regex::Regex;
use regex::bytes::Regex;

use error::Error;

#[macro_use]
extern crate lazy_static;

use requests::rss_parse::{self, get_xml, AnnounceComponents};
use requests::url_encoding::{Url, hex_to_char};
use hashbrown::HashMap;

use std::io::prelude::*;
use std::fs::File;

use bencode::Bencode;
use bencode::ToBencode;
use serde_urlencoded::ser;

use hashbrown::HashSet;

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

fn write_torrent(read: &str, write: &str) -> Result<(), Error> {
    let mut torrent = read_torrent::Torrent::new_file(&read)?;
	let x = torrent.to_bencode().to_bytes()?;
	let mut file = File::create(&write)?;

	file.write(&x);

	Ok(())

}

fn read_data (loc: &str ) -> Result<(), Error> {
	let mut buffer = Vec::new();

	let mut file = File::open(&loc)?;


	file.read_to_end(&mut buffer);
	let ans = read_torrent::sha1(&buffer);

	dbg!{ans};
	Ok(())
}


fn test_funct(instr: &str) -> Result<String, String>{
	Ok("test".to_string())
}


const torrents_dir : &str= r"C:\Users\Brooks\github\nyaa_tracker\torrents\";
const si_rss : &str = r"https://nyaa.si/?page=rss";
const pantsu_rss : &str = r"https://nyaa.pantsu.cat/feed?";


fn main() {

	let previous = utils::info_hash_set(torrents_dir);
	let ans = get_xml(&si_rss, &previous);
	// let  ans =get_xml(&pantsu_rss, &previous);

	// let ans = previous.contains("tset");
	dbg!{ans};
	
	
	// let read = "C:\\Users\\Brooks\\github\\nyaa_tracker\\torrents";
	// let set =utils::info_hash_set(&torrents_dir);
	// dbg!{set};
	// let k = read_torrent::Torrent::new_file(&read);
	// dbg!{k.unwrap().info};


	// let k =utils::torrents_with_hashes("C:\\Users\\Brooks\\github\\nyaa_tracker\\torrents\\");
	// dbg!{&k[0]};

	// utils::check_hashes(r"C:\Users\Brooks\github\nyaa_tracker\torrents");
	// let x = "test_str";
	// parse!(test_funct, x);

}

