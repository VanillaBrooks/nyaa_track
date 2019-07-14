// use super::super::read::{announce_components, announce_result};
use super::super::read::{AnnounceComponents, GenericData};

use futures::sync::mpsc;
use futures::{Future, Stream};


use std::sync::Arc;
use parking_lot::RwLock;
use hashbrown::HashSet;

use std::time::Duration;

use tokio::prelude::*;

/// starts task for cycling through scrapes
/// recieves announce component and spawns a request after a timer
/// That spawned task will pass ownership of itself back to this task and be re-spawned
pub fn start_scrape_cycle_task(
	rx_to_scrape: mpsc::Receiver<AnnounceComponents>,
	tx_to_scrape: mpsc::Sender<AnnounceComponents>,
	tx_generic: mpsc::Sender<GenericData>
	) -> () {

	dbg!{"starting scrape task"};

	let allow_new_scrapes = 
		rx_to_scrape.throttle(Duration::from_millis(95)).for_each(move |ann|{

			ann.get(tx_to_scrape.clone(), tx_generic.clone());

			Ok(())
		})
		.map_err(|e| println!{"ERROR MAIN SCRAPE {:?}",e});
		
		
	tokio::spawn(allow_new_scrapes);
}


pub fn filter_new_announces(
	rx_filter: mpsc::Receiver<AnnounceComponents>,
	tx_to_scrape: mpsc::Sender<AnnounceComponents>,
	tx_generic: mpsc::Sender<GenericData>,
	previous_lock: Arc<RwLock<HashSet<String>>>
	) -> () {
	
	dbg!{"starting filter task"};

	let filter = 
		rx_filter.for_each(move |ann| {
			let hash_ptr = Arc::into_raw(ann.info_hash.clone());
			let hash = unsafe{(*hash_ptr).clone()};

			{
				println!{"writing new value {:?}", &hash}
				let mut previous = previous_lock.write();
				previous.insert(hash);
			}

			unsafe{ Arc::from_raw(hash_ptr) };

			ann.get(tx_to_scrape.clone(), tx_generic.clone());

			Ok(())
		})
		.map_err(|e| println!{"ERROR MAIN FILTERING {:?}",e});

	tokio::spawn(filter);
}