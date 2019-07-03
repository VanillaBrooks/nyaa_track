// use super::super::read::{announce_components, announce_result};
use super::super::read::{AnnounceResult ,AnnounceComponents, GenericData};
use super::super::database;

use futures::sync::mpsc;
use futures::{Future, Stream};
use futures::sink::Sink;

use tokio::timer::Timeout;

use super::super::error::*;
use super::super::utils;

use std::sync::Arc;
use parking_lot::RwLock;
use hashbrown::HashSet;

use std::thread;
use std::time::Duration;
use tokio::timer;


/// starts task for cycling through scrapes
/// recieves announce component and spawns a request after a timer
/// That spawned task will pass ownership of itself back to this task and be re-spawned
pub fn start_scrape_cycle_task(
	rx_to_scrape: mpsc::Receiver<AnnounceComponents>,
	tx_to_scrape: mpsc::Sender<AnnounceComponents>,
	tx_generic: mpsc::Sender<GenericData>
	) -> () {


	let inter_wrap = timer::Interval::new_interval(Duration::from_millis(100));

	let allow_new_scrapes = 
		rx_to_scrape.for_each(move |ann|{

			thread::sleep(Duration::from_millis(100));
			ann.get(tx_to_scrape.clone(), tx_generic.clone());

			Ok(())
		});
		
		
	tokio::spawn(allow_new_scrapes);
}


pub fn filter_new_announces(
	rx_filter: mpsc::Receiver<AnnounceComponents>,
	tx_to_scrape: mpsc::Sender<AnnounceComponents>,
	tx_generic: mpsc::Sender<GenericData>,
	previous_lock: Arc<RwLock<HashSet<String>>>
	) -> () {
	
	let filter = 
		rx_filter.for_each(move |new_announce_struct| {
			println!{"new value sent to filter"}

			{
				let mut previous = previous_lock.write();
				previous.insert(new_announce_struct.info_hash.to_string());
			}

			new_announce_struct.get(tx_to_scrape.clone(), tx_generic.clone());

			Ok(())
		})
		.map(|x| ());

	tokio::spawn(filter);
}