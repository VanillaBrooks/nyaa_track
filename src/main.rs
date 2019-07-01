
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

macro_rules! rss_check {
	($timer:ident, $announce:ident, $prev:ident) => {
		if $timer.allow_check() {

			match rss_parse::get_xml($timer.url, &mut $prev) {
				Ok(data) => {
					let mut filt_data = utils::filter_nyaa_announces(data.good);
					$announce.append(&mut filt_data);
				},
				Err(err) => () //TODO log the error 
			}
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

	let (tx_gen, rx_gen) = mpsc::channel::<read::GenericData>(1_024*100*1000);			// 100 MB cache
	let (tx_ann, rx_ann) = mpsc::channel::<read::AnnounceComponents>(1_024*100*1000); 	// 100 MB cache

	let all_announce_components = Arc::new(RwLock::new(Vec::new()));
	let previous = Arc::new(RwLock::new(utils::info_hash_set(TORRENTS_DIR)));
	{
		let len = previous.read().len();
		let len2 = all_announce_components.read().len();
		println!{":::::::::::::::::::::::starting length previous : {} announce: {} ", len, len2}
	}


	let mut database_announces = database::pull_data::database_announce_components().expect("sync database pull error");
		// .into_iter().take(10).collect();

	{
		let mut ann = all_announce_components.write();
		ann.append(&mut database_announces);
	}

	// all_announce_components.append(&mut database_announces);


	// let mut si_timer = rss_parse::Timer::new(60*5, SI_RSS);
	// let mut pantsu_timer = rss_parse::Timer::new(60*5, PANTSU_RSS);

	let runtime = 
		lazy(move || {
			/*
				updater for the previous hashmap 
				updater for the announce vector
				this is the only area lock.write() should be called
			*/
			let updater_previous_clone = previous.clone();
			let all_announce_clone = all_announce_components.clone();

			let new_ann = 
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

					println!{" added new values to hashmap"}

					Ok(())
				})
				.map(|x| println!{"drop updater for hashmap / announce vector "});
			tokio::spawn(new_ann);

			/*
				Start async database
			*/
			database::connection::start_async(rx_gen);

			

			loop {
				let tx_gen_clone = tx_gen.clone();
				let tx_ann_clone = tx_ann.clone();

				let rss_previous_clone = previous.clone();
				let rss_fut = rss_parse::get_xml(SI_RSS, rss_previous_clone, tx_ann_clone)
					.map(|x| println!{"finished rss write"})
					.map_err(|x|  println!{"error with rss parse"});
				tokio::spawn(rss_fut);
				/*
					fetch rss
				*/
				
				/*
					Make get requests
				*/
				let requests_announce_clone = all_announce_components.clone();
				requests::tracking::announce_all_components(requests_announce_clone,tx_gen_clone);


				{
				let len = previous.read().len();
				let len2 = all_announce_components.read().len();
				println!{":::::::::::::::::::::::ending length previous : {} announce: {} ", len, len2}
				}


			}


			Ok(())
		});

	dbg!{"spawning runtime"};
	tokio::run (runtime);

}