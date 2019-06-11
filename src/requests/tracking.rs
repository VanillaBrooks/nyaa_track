// use super::super::read::{announce_components, announce_result};
use super::super::read::{AnnounceResult ,AnnounceComponents, GenericData};
use super::super::database;

use super::super::error::*;

pub fn announce_all_components(components: &mut Vec<AnnounceComponents>) -> Vec<GenericData> {
	let mut announce_results = Vec::with_capacity(components.len()/10);
	let start_len = components.len() as i32;

	for item in components {
		match item.scrape() {
			Ok(announce) => {
				dbg!{&announce};
				announce_results.push(announce);
			}
			Err(error) => () // TODO: log the error here
		}
	}
	let res = announce_results.len() as i32;

	println!{"Announce Results:\nInput Count:\t{}\nSuccessful Announces:\t{}\nBad Announces:\t{}",start_len, res, (start_len-res)}
	return announce_results
}


pub fn update_database(stats: &Vec<GenericData>) -> Result<(), Error> {
	let conn = database::connection::start()?;
	
	let prepare_info = conn.prepare("INSERT INTO info (info_hash, announce_url, creation_date, title) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING").unwrap();
	let prepare_data = conn.prepare("with ref_id as (select id from info where info_hash=$1 and announce_url =$2) insert into stats (stats_id, downloaded, seeding, incomplete, poll_time) values ((select * from ref_id), $3,$4,$5,$6)").unwrap();
	
	for res in stats{

		match prepare_info.execute(&[&res.hash, &res.url, &res.creation_date, &res.title]){
			Ok(_) => (),
			Err(error) => () // TODO log error
		}

		match prepare_data.execute(&[&res.hash, &res.url, &res.downloaded, &res.complete, &res.incomplete, &res.poll_time]) {
			Ok(_) => (),
			Err(error) => () // TODO: log error
		}

	}

	Ok(())
}

    // pub complete: i64, //seeds
    // pub incomplete: i64, // downloading now
    // pub downloaded: i64,  // snatches