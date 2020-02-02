// use super::super::read::{announce_components, announce_result};
use super::super::database::connection;
use super::super::read::AnnounceComponents;

use futures::channel::mpsc;
use futures::StreamExt;

use parking_lot::RwLock;
use std::collections::HashSet;
use std::sync::Arc;
use tokio;

/// starts task for cycling through scrapes
/// recieves announce component and spawns a request after a timer
/// That spawned task will pass ownership of itself back to this task and be re-spawned
pub fn start_scrape_cycle_task(
    mut rx_to_scrape: mpsc::Receiver<AnnounceComponents>,
    tx_to_scrape: mpsc::Sender<AnnounceComponents>,
    tx_generic: mpsc::Sender<connection::DatabaseUpsert>,
) {
    let fut = async move {
        // dbg! {"|||||||||||||||||||| starting scrape task"};
        while let Some(ann) = rx_to_scrape.next().await {
            // dbg! {"running new scrape"};
            ann.get(tx_to_scrape.clone(), tx_generic.clone()).await;
            tokio::time::delay_for(std::time::Duration::from_millis(50)).await;
        }
    };

    tokio::spawn(fut);
}

pub async fn filter_new_announces<T: Send + Sync + std::hash::BuildHasher + 'static>(
    mut rx_filter: mpsc::Receiver<AnnounceComponents>,
    tx_to_scrape: mpsc::Sender<AnnounceComponents>,
    tx_generic: mpsc::Sender<connection::DatabaseUpsert>,
    previous_lock: Arc<RwLock<HashSet<String, T>>>,
) {
    let fut = async move {
        // dbg! {"|||||||||||||||||||| started filter new announce. nothing recieved yet"};
        while let Some(ann) = rx_filter.next().await {
            // this is blocked to prevent .await lifetime issues
            {
                // dbg! {"new announce recieved"};
                let mut previous = previous_lock.write();
                previous.insert(ann.info_hash.to_string());
            }

            ann.get(tx_to_scrape.clone(), tx_generic.clone()).await;
        }
    };

    tokio::spawn(fut);
}
