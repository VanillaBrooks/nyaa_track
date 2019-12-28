use super::super::error::*;
use super::super::read::AnnounceComponents;
use super::connection;

use tokio_postgres::{self, NoTls};

pub async fn database_announce_components() -> Result<Vec<AnnounceComponents>, Error> {
    let db_url = connection::DatabaseConfig::new().connection_url();

    let (client, _connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    let pull = client.prepare("SELECT info_hash, creation_date, title, announce_url FROM info WHERE announce_url='http://nyaa.tracker.wf:7777/announce'").await?;

    let count = client.query("SELECT COUNT(*) FROM info", &[]).await?;
    let len: i64 = count.get(0).unwrap().get(0);
    let mut res_vec: Vec<AnnounceComponents> = Vec::with_capacity(len as usize);

    for row in client.query(&pull, &[]).await? {
        let hash = row.get(0);
        let date = row.get(1);
        let title = row.get(2);
        let url = row.get(3);
        match AnnounceComponents::new(Some(url), hash, date, title) {
            Ok(data) => res_vec.push(data),
            Err(e) => println! {"serialize to AnnounceComponents error: {:?}", e},
        }
    }

    Ok(res_vec)
}
