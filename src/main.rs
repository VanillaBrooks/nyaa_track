// #[allow(unused_imports)]
#![allow(unused_must_use)]
// #[macro_use]
pub mod database;
pub mod error;
pub mod read;
pub mod requests;
pub mod utils;

use database::connection;

use requests::rss_parse;

use parking_lot::RwLock;
use std::sync::Arc;

use futures::channel::mpsc;
use futures::SinkExt;
// use futures::stream::Stream;
use tokio;

use hashbrown::HashSet;

#[allow(dead_code)]
const SI_RSS: &str = r"https://nyaa.si/?page=rss";
#[allow(dead_code)]
const PANTSU_RSS: &str = r"https://nyaa.pantsu.cat/feed?";

/// Macro instead of function since this will reduce the ammount of
/// clones needed (ownership is retained since it is inlined)
macro_rules! rss_check {
    ($timer:ident, $previous:ident, $tx_ann:ident) => {
        if $timer.allow_check() {
            dbg! {"running rss check"};
            let rss_previous_clone = $previous.clone();
            let tx_ann_clone = $tx_ann.clone();
            let url_clone = $timer.url.clone();

            let rss_fut = async move {
                let res = rss_parse::get_xml(url_clone, rss_previous_clone, tx_ann_clone).await;

                if res.is_ok() {
                    println! {"finished rss write"}
                } else {
                    println! {" error with rss task"}
                }
            };
            tokio::spawn(rss_fut);
        }
    };
}

/// start the asynchronous database with handler
fn start_database_task(rx_generic: mpsc::Receiver<connection::DatabaseUpsert>) {
    database::connection::start_async(rx_generic);
}

#[tokio::main]
async fn main() {
    let size = 1_024 * 1_024 * 100; // 100 MB cache
    let (tx_generic, rx_generic) = mpsc::channel::<connection::DatabaseUpsert>(size); // to database
    let (mut tx_to_scrape, rx_to_scrape) = mpsc::channel::<read::AnnounceComponents>(size); // to the scrape / announce cycle
    let (tx_filter, rx_filter) = mpsc::channel::<read::AnnounceComponents>(size); // to the step between rss and announce

    let mut previous_hashes = HashSet::<String>::new();
    let mut ann_components =
        database::pull_data::database_announce_components().await.expect("sync database pull error");
    for _ in 0..ann_components.len() {
        let comp = ann_components.remove(0);
        previous_hashes.insert(comp.info_hash.to_string());
        tx_to_scrape.send(comp).await;
    }
    // .expect("sync database pull error")
    // .into_iter()
    // // .take(1)
    // .for_each(move |x| {
    //     previous_hashes.insert(x.info_hash.to_string());
    //     tx_to_scrape.clone().send(x).await;
    // });

    let previous = Arc::new(RwLock::new(previous_hashes));

    dbg! {"finished adding to queue"};

    // tracking for nyaa.si
    let mut si_timer = rss_parse::Timer::new(60, SI_RSS);

    // core logic of the program

    let runtime = async move {
        requests::tracking::filter_new_announces(
            rx_filter,
            tx_to_scrape.clone(),
            tx_generic.clone(),
            previous.clone(),
        )
        .await;

        requests::tracking::start_scrape_cycle_task(rx_to_scrape, tx_to_scrape, tx_generic);

        start_database_task(rx_generic);

        loop {
            /*
                fetch rss when the timer on it allows us to do so.
            */
            rss_check! {si_timer, previous, tx_filter};
        }
    };

    tokio::spawn(runtime);
}
