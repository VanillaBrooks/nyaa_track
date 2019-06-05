use std::time;

use super::super::read::{announce_components, announce_result};
use announce_result::AnnounceResult;
use announce_components::AnnounceComponents;

use super::super::database;

pub fn announce_all_components(components: &mut Vec<AnnounceComponents>) -> Vec<AnnounceResult> {
	let mut announce_results : Vec<AnnounceResult> = Vec::with_capacity(components.len()/10);

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
	let conn = database::connection::start().unwrap();				// FIX ME
	let prepare_info = conn.prepare("INSERT INTO info (info_hash, announce_url, creation_date, title) VALUES ($1, $2, $3, $4)").unwrap();
	let prepare_data = conn.prepare("INSERT INTO stats (stats_id, downloaded, seeding, incomplete) VALUES (id, $1, $2, $4) WHERE stats_id IN (SELECT id FROM info WHERE info_hash = $5").unwrap();
	for res in stats{
		prepare_info.execute(&[&res.info_hash, &res.announce_url, &res.creation_date, &res.title]);
		prepare_data.execute(&[&res.data.downloaded, &res.data.complete, &res.data.incomplete, &res.info_hash]);
	}
}