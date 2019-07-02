
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
use futures::{Future, Stream};
// use futures::stream::Stream;

use hashbrown::HashSet;

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

#[allow(dead_code)]const TORRENTS_DIR : &str= r"C:\Users\Brooks\github\nyaa_tracker\torrents\";
#[allow(dead_code)]const SI_RSS: &str = r"https://nyaa.si/?page=rss";
#[allow(dead_code)]const PANTSU_RSS : &str = r"https://nyaa.pantsu.cat/feed?";
#[allow(dead_code)]const TEST_FILE :&str=  r"C:\Users\Brooks\Downloads\test.txt";


fn main() {

	let (tx_gen, rx_gen) = mpsc::channel::<read::GenericData>(1_024*1_024*100);			// 100 MB cache
	let (tx_ann, rx_ann) = mpsc::channel::<read::AnnounceComponents>(1_024*1_024*100); 	// 100 MB cache
	let (tx_allow_ann, rx_allow_ann) = mpsc::channel::<bool>(1024); 					// 1   MB cache

	let all_announce_components : Arc<RwLock<Vec<AnnounceComponents>>>= Arc::new(RwLock::new(Vec::new()));


	let mut database_announces = database::pull_data::database_announce_components().expect("sync database pull error");
	let previous_hashes = database_announces.iter().map(|x| x.info_hash.to_string()).collect::<HashSet<_>>();
		// .into_iter().take(100).collect();
	let mut previous = Arc::new(RwLock::new(previous_hashes));

	// let previous = Arc::new(RwLock::new(HashSet::new()));
	
	{
		let mut ann = all_announce_components.write();
		ann.append(&mut database_announces);
	}

	// {
	// 	let len = previous.read().len();
	// 	// let len2 = all_announce_components.read().len();
	// 	println!{":::::::::::::::::::::::starting length previous : {} announce: {} ", len, len2}
	// }

	// all_announce_components.append(&mut database_announces);


	let mut si_timer = rss_parse::Timer::new(60, SI_RSS);
	// let mut pantsu_timer = rss_parse::Timer::new(60*5, PANTSU_RSS);

	let runtime = 
		lazy(move || {
			/*
				updater for the previous hashmap 
				updater for the announce vector
				this is the only area lock.write() should be called
			*/
			let updater_previous_clone = previous.clone();					// hashset of previously downloaded hashes
			let all_announce_clone = all_announce_components.clone();		// arc mutex of announce components

			let update_announce_vec = 
				rx_ann.for_each(move |announce| {

					{ //scope for locks
						let mut previous_lock = updater_previous_clone.write();

						// if the hash set does not have the hash insert to announces
						if !previous_lock.contains(&announce.info_hash){
							previous_lock.insert(announce.info_hash.clone());
							let mut announces_lock = all_announce_clone.write();
							announces_lock.push(announce);
						}
						
					} // drop lock

					println!{" added new values to hashmap"};

					Ok(())
				})
				.map(|x| println!{"drop updater for hashmap / announce vector "});
			tokio::spawn(update_announce_vec);

			/*
				task for starting new scrapes once a acceptance message comes from the previous scrape
			*/

			let fut_tx_allow_ann = tx_allow_ann.clone();				// allows an announce seq to start
			let fut_tx_gen = tx_gen.clone();							// passes data to database struct
			let fut_all_announce = all_announce_components.clone();		// arc mutex of announce components

			let allow_new_scrapes = 
				rx_allow_ann.for_each(move |_| {
					
					println!{":::::::::: scrape has ended, we are allowing a new scrape"};

					let arc_announce = fut_all_announce.clone();
					let tx_gen_clone = fut_tx_gen.clone();
					let tx_allow_ann_clone = fut_tx_allow_ann.clone();

					requests::tracking::announce_all_components(arc_announce, tx_gen_clone, tx_allow_ann_clone);
					Ok(())
				})
				.map(|_| println!{"dropped allowance for new scrapes"});
			tokio::spawn(allow_new_scrapes);


			/*
				Start async database
			*/
			database::connection::start_async(rx_gen);

			/*

				start initial scrape

			*/
			let tx_gen_clone = tx_gen.clone();
			let tx_allow_ann_clone = tx_allow_ann.clone();
			let requests_announce_clone = all_announce_components.clone();
			requests::tracking::announce_all_components(requests_announce_clone,tx_gen_clone, tx_allow_ann_clone);
			

			loop {
				/*
					fetch rss when the timer on it allows us to do so.
				*/

				rss_check!{si_timer, previous, tx_ann};

			}

			Ok(())
		});

	dbg!{"spawning runtime"};
	tokio::run(runtime);

}