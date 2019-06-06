
#[allow(unused_imports)]
use read::{announce_components, announce_result, torrent};
use announce_components::AnnounceComponents;
// use announce_result::AnnounceResult;
use torrent::Torrent;

pub mod utils;
pub mod read;
pub mod requests;
pub mod error;
pub mod database;


use error::Error;

#[macro_use]
extern crate lazy_static;

use requests::rss_parse::{self, get_xml};

use std::io::prelude::*;
use std::fs::File;

use bencode::ToBencode;

use std::time;

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

#[allow(dead_code)]
const TORRENTS_DIR : &str= r"C:\Users\Brooks\github\nyaa_tracker\torrents\";
#[allow(dead_code)]
const SI_RSS: &str = r"https://nyaa.si/?page=rss";
#[allow(dead_code)]
const PANTSU_RSS : &str = r"https://nyaa.pantsu.cat/feed?";
#[allow(dead_code)]
const TEST_FILE :&str=  r"C:\Users\Brooks\Downloads\test.txt";

fn diff(title: &str, t1: &time::Instant, t2: &time::Instant) {
	println!{"{}", title};
	dbg!{(*t2-*t1).as_millis()};
}


fn main() {


	let mut all_announce_components : Vec<AnnounceComponents>= Vec::new();
	let mut previous = utils::info_hash_set(TORRENTS_DIR);

	let mut file_announce_comp = utils::nyaa_si_announces_from_files(TORRENTS_DIR);
	let mut database_announces = database::pull_data::database_announce_components().unwrap();
	all_announce_components.append(&mut file_announce_comp);

	loop {
		// pull rss from nyaa.si
		let ans = get_xml(&SI_RSS, &mut previous);
		match ans {
			Ok(data_struct) => {all_announce_components.append(&mut utils::filter_nyaa_announces(data_struct.good))},
			Err(_) => ()
		}
		
		// pull rss from nyaa.pantsu.cat
		let ans =get_xml(&PANTSU_RSS, &mut previous);
		match ans {
			Ok(data_struct) => {all_announce_components.append(&mut utils::filter_nyaa_announces(data_struct.good))},
			Err(_) => ()
		}


		// start announcing 
		let ann_start = time::Instant::now();
		let announces = requests::tracking::announce_all_components(&mut all_announce_components);

		// update database for the announce information
		let db_start = time::Instant::now();
		match requests::tracking::update_database(&announces) {
			Ok(_)=> (),
			Err(error) => { //todo : log the error here
				println!{"error with the database"}
			}
		}

		let db_end = time::Instant::now();

		diff("time to announce:" , &ann_start, &db_start);
		diff("time updating psql:", &db_start, &db_end);
		println!{"\n"}
	}


}

