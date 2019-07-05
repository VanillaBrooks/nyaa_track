
// #[allow(unused_imports)]
use read::{Torrent, AnnounceResult, AnnounceComponents};


// #[macro_use]
pub mod read;
pub mod utils;
pub mod requests;
pub mod error;
pub mod database;


use error::*;

use requests::rss_parse;

use std::sync::Arc;
use parking_lot::RwLock;

use futures::sync::mpsc;
use futures::future::lazy;
use futures::{Future, Stream, Sink};
// use futures::stream::Stream;

use hashbrown::HashSet;

use read::GenericData;


#[allow(dead_code)]const TORRENTS_DIR : &str= r"C:\Users\Brooks\github\nyaa_tracker\torrents\";
#[allow(dead_code)]const SI_RSS: &str = r"https://nyaa.si/?page=rss";
#[allow(dead_code)]const PANTSU_RSS : &str = r"https://nyaa.pantsu.cat/feed?";
#[allow(dead_code)]const TEST_FILE :&str=  r"C:\Users\Brooks\Downloads\test.txt";

/// Macro instead of function since this will reduce the ammount of 
/// clones needed (ownership is retained since it is inlined)
macro_rules! rss_check {
	($timer:ident, $previous:ident, $tx_ann:ident) => {
		if $timer.allow_check() {
			dbg!{"running rss check"};
			let rss_previous_clone = $previous.clone();
			let tx_ann_clone = $tx_ann.clone();

			let rss_fut = rss_parse::get_xml(
				$timer.url,
				rss_previous_clone, 
				tx_ann_clone)
					.map(|_| println!{"finished RSS write"})
					.map_err(|e| println!{"Error with RSS task "});
			tokio::spawn(rss_fut);

		}
	};
}


#[allow(dead_code)]
fn load_problem_hash(hash: &str)  {
	let mut file = TORRENTS_DIR.clone().to_string();
	file.push_str(&hash);
	file.push_str(".torrent");
	let torrent = Torrent::new_file(&file);

	match torrent {
		Ok(x)=> {
			// let i = x.info;
			dbg!{x};
		},
		Err(x)=> println!{"error loading hash {} : {:?}", hash, x}
	}
}

/*
	Start async database
*/
fn start_database_task(rx_generic: mpsc::Receiver<GenericData>) {
			database::connection::start_async(rx_generic);
}



fn main() {
	let size = 1_024*1_024*100;														// 100 MB cache
	let (tx_generic, rx_generic) = mpsc::channel::<read::GenericData>(size);				// to database		
	let (tx_to_scrape, rx_to_scrape) = mpsc::channel::<read::AnnounceComponents>(size); 	// to the scrape / announce cycle
	let (tx_filter, rx_filter) = mpsc::channel::<read::AnnounceComponents>(size);			// to the step between rss and announce


	let mut previous_hashes = HashSet::<String>::new();
	database::pull_data::database_announce_components()
		.expect("sync database pull error")
		.into_iter()
		// .take(1)
		.for_each(|x| {
			previous_hashes.insert(x.info_hash.to_string());
			tx_to_scrape.clone().send(x).wait();
			});

	let previous = Arc::new(RwLock::new(previous_hashes));
	// let previous = Arc::new(RwLock::new(HashSet::new()));

	dbg!{"finished adding to queue"};

	let mut si_timer = rss_parse::Timer::new(60, SI_RSS);

	let runtime = 
		lazy(move || {

			requests::tracking::filter_new_announces(
				rx_filter, 
				tx_to_scrape.clone(), 
				tx_generic.clone(), 
				previous.clone());
			
			requests::tracking::start_scrape_cycle_task(rx_to_scrape, tx_to_scrape, tx_generic);

			start_database_task(rx_generic);


			loop {
				/*
					fetch rss when the timer on it allows us to do so.
				*/
				rss_check!{si_timer, previous, tx_filter};
			}

			Ok(())
		});

	dbg!{"spawning runtime"};
	tokio::run(runtime);

}
