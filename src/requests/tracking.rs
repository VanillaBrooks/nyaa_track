
use std::time;

use super::rss_parse::{AnnounceComponents};
use super::super::read_torrent;

pub fn announce_components(components: &mut Vec<AnnounceComponents>) -> () {
	let mut announce_results : Vec<read_torrent::Announce> = Vec::with_capacity(components.len());

	for item in components {

		match item.announce() {
			Ok(announce) => {
				announce_results.push(announce);
			}
			Err(error) => () // TODO: log the error here
		}

	}

	println!("final torrents");
	dbg!{announce_results};
}