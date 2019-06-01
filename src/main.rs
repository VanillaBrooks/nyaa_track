mod read_torrent;
use read_torrent::Torrent;

pub mod requests;
pub mod error;
pub mod utils;

use error::Error;

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

//6f7421ac8957e4a20b971d8839707cc58e55615a

fn main() {
    let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
    git_torrents.push_str("1147506.torrent");


	let nyaa = "https://nyaa.si/?page=rss";
	let data = get_xml(nyaa);
	dbg!{data};
}


//8463057ea30edd86f3968c57ca4658090c616382