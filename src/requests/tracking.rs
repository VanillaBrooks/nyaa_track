use std::time;

use super::super::read_torrent;

pub fn announce_components(components: &mut Vec<read_torrent::AnnounceComponents>) -> Vec<read_torrent::Announce> {
	let mut announce_results : Vec<read_torrent::Announce> = Vec::with_capacity(components.len());

	for item in components {
		match item.announce() {
			Ok(announce) => {
				announce_results.push(announce);
			}
			Err(error) => () // TODO: log the error here
		}
	}

	return announce_results
}


pub fn update_database(stats: &Vec<read_torrent::Announce>) -> () {
	unimplemented!()
}