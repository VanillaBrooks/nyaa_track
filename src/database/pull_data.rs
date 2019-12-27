use super::super::error::*;
use super::super::read::AnnounceComponents;
use super::connection;

pub fn database_announce_components() -> Result<Vec<AnnounceComponents>, Error> {
    let mut client = connection::start_sync()?;
    // let pull = conn.prepare("SELECT info_hash, creation_date, title, announce_url FROM info WHERE announce_url='http://nyaa.tracker.wf:7777/announce'")?;
    let pull =
        client.prepare("SELECT info_hash, creation_date, title, announce_url FROM data_to_track")?;

    let count = client.query("SELECT COUNT(*) FROM info", &[])?;
    let len: i64 = count.iter().nth(0).unwrap().get(0);
    let mut res_vec: Vec<AnnounceComponents> = Vec::with_capacity(len as usize);

    for row in client.query(&pull, &[])? {
        let hash = row.get(0);
        let date = row.get(1);
        let title = row.get(2);
        let url = row.get(3);
        match AnnounceComponents::new(Some(url), hash, date, title) {
            Ok(data) => res_vec.push(data),
            Err(e) => println! {"serialize to AnnounceComponents error: {:?}", e},
        }
    }

    return Ok(res_vec);
}
