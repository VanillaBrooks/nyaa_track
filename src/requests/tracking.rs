use std::time;

use super::super::read::{announce_components, announce_result};
use announce_result::AnnounceResult;
use announce_components::AnnounceComponents;

pub fn announce_components(components: &mut Vec<AnnounceComponents>) -> Vec<AnnounceResult> {
	let mut announce_results : Vec<AnnounceResult> = Vec::with_capacity(components.len());

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


pub fn update_database(stats: &Vec<AnnounceResult>) -> () {
	unimplemented!()
}