// use super::super::read::{announce_components, announce_result};
use super::super::read::{AnnounceResult ,AnnounceComponents, GenericData};
use super::super::database;

use futures::sync::mpsc;
use futures::Future;
use futures::sink::Sink;

use tokio::timer::Timeout;

use super::super::error::*;
use super::super::utils;

use std::sync::Arc;
use parking_lot::RwLock;

use std::thread;
use std::time::Duration;

pub fn announce_all_components(
	components_arc: Arc<RwLock<Vec<AnnounceComponents>>>,
	tx: mpsc::Sender<GenericData>,
	tx_allow_spawn: mpsc::Sender<bool>
	) -> () {

	// dbg!{"starting to announce all comp"};
	let mut drop_indicies = Vec::new();
	{
		let components = components_arc.read();
		let current_time = utils::get_unix_time();
		
		for i in 0..components.len() {
			let ann_cmp = components.get(i).expect("components index out of bounds which should not happen");
			let time_diff =  (current_time - ann_cmp.creation_date) / 86400;
			
			// drop items older than 4 weeks
			if time_diff > 4*7 {
				drop_indicies.push(i);
				continue
			}
			
			let tx_clone = tx.clone();
			
			let fut = Timeout::new(ann_cmp.get() , Duration::from_secs(10))
				.map(move |x| {
					// println!{"success in scrape data"}
					tx_clone.send(x).wait();
					})
				.map_err(|e| println!{"scrape err: {:?}",e});
				// .map_err(|x| println!{"there was an error with the scrape: {:?}", x});


			// dbg!{"spawned fut"};
			tokio::spawn(fut);

			thread::sleep(Duration::from_millis(50));
		}
	} // read lock is dropped 

	let mut components = components_arc.write();

	// the write lock has been acquired, which means that there are no longer read references to the announce components, we can now send the signal to start a new scrape
	// Note that this would mean that the database has to insert all the data, but given that this happens asyncronously it should not be a large portion of overhead
	tx_allow_spawn.send(true).wait();

	// iterate from top to bottom so we dont mess up the future indexing
	// removes data from the vector that is expired
	for i in drop_indicies.iter().rev() {
		components.remove(*i);

	}

}