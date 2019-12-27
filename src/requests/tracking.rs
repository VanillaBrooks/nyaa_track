// use super::super::read::{announce_components, announce_result};
use super::super::database::connection;
use super::super::read::{AnnounceComponents};

use futures::channel::mpsc;
use futures::StreamExt;

use hashbrown::HashSet;
use parking_lot::RwLock;
use std::sync::Arc;
use super::super::error;

/// starts task for cycling through scrapes
/// recieves announce component and spawns a request after a timer
/// That spawned task will pass ownership of itself back to this task and be re-spawned
pub fn start_scrape_cycle_task(
    mut rx_to_scrape: mpsc::Receiver<AnnounceComponents>,
    tx_to_scrape: mpsc::Sender<AnnounceComponents>,
    tx_generic: mpsc::Sender<connection::DatabaseUpsert>,
){
    dbg! {"starting scrape task"};

    let fut = async move {
        while let Some(ann) = rx_to_scrape.next().await {
            ann.get(tx_to_scrape.clone(), tx_generic.clone()).await;
        }
    };

    tokio::spawn(fut);
}

pub async fn filter_new_announces(
    mut rx_filter: mpsc::Receiver<AnnounceComponents>,
    tx_to_scrape: mpsc::Sender<AnnounceComponents>,
    tx_generic: mpsc::Sender<connection::DatabaseUpsert>,
    previous_lock: Arc<RwLock<HashSet<String>>>,
) -> Result<(), error::Error> {
    dbg! {"starting filter task"};


    while let Some(ann) = rx_filter.next().await {
        let hash_ptr = Arc::into_raw(ann.info_hash.clone());
        let hash = unsafe { (*hash_ptr).clone() };

        {
            println! {"writing new value {:?}", &hash}
            let mut previous = previous_lock.write();
            previous.insert(hash);
        }

        unsafe { Arc::from_raw(hash_ptr) };

        ann.get(tx_to_scrape.clone(), tx_generic.clone()).await;
    }

    dbg!{"returning from filteR_new_announces. this should not happen"};

    Ok(())
}
