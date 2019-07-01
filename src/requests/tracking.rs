// use super::super::read::{announce_components, announce_result};
use super::super::read::{AnnounceResult ,AnnounceComponents, GenericData};
use super::super::database;

use futures::sync::mpsc;
use futures::Future;
use futures::sink::Sink;

use super::super::error::*;
use super::super::utils;

use std::sync::Arc;
use parking_lot::RwLock;

use std::thread;
use std::time::Duration;

pub fn announce_all_components<'a>(
	components_arc: Arc<RwLock<Vec<AnnounceComponents>>>,
	tx: mpsc::Sender<GenericData>
	) -> () {

	dbg!{"starting to announce all comp"};
	let mut drop_indicies = Vec::new();
	{
		let mut components = components_arc.read();

		let current_time = utils::get_unix_time();
		


		for i in 0..components.len() {
			let ann_cmp = components.get(i).expect("components index out of bounds which should not happen");
			let time_diff =  (current_time - ann_cmp.creation_date) / 86400;
			
			// drop items older than 4 weeks
			if time_diff > 4*7 {
				drop_indicies.push(i);
				continue
			}
			
			let mut tx_clone = tx.clone();
			
			let fut = ann_cmp.get()
				.map(move |x| {
					// println!{"success in scrape data"}
					let mut sink = tx_clone.send(x).wait();
					})
		
				.map_err(|x| println!{"there was an error with the scrape: {:?}", x});


			// dbg!{"spawned fut"};
			tokio::spawn(fut);

			thread::sleep(Duration::from_millis(100));
		}
	} // read lock is dropped 

	let mut components = components_arc.write();
	// iterate from top to bottom so we dont mess up the future indexing
	for i in drop_indicies.iter().rev() {
		components.remove(*i);

	}

}


pub fn update_database(stats: &Vec<GenericData>) -> Result<(), Error> {
	let conn = database::connection::start_sync()?;
	
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