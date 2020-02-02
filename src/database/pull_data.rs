use super::super::error::*;
use super::super::read::AnnounceComponents;
use super::connection;
use tokio;
use tokio_postgres::NoTls;

pub async fn database_announce_components() -> Result<Vec<AnnounceComponents>, Error> {
    dbg! {"fetching announce components"};

    let db_url = connection::DatabaseConfig::new().connection_url();

    dbg! {&db_url};

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let pull = client.prepare("SELECT info_hash, creation_date, title, announce_url FROM info WHERE announce_url='http://nyaa.tracker.wf:7777/announce'").await?;

    let count = client.query("SELECT COUNT(*) FROM info", &[]).await?;
    let len: i64 = count.get(0).unwrap().get(0);
    let mut res_vec: Vec<AnnounceComponents> = Vec::with_capacity(len as usize);

    // dbg! {"before loop"};

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
    // dbg! {"after loop"};

    Ok(res_vec)
}
