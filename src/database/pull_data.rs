use super::super::error::*;
use super::super::read::AnnounceComponents;
use super::connection;

macro_rules! construct {
    ($type:ident) => {
        let conn = connection::start_sync()?;
        // let pull = conn.prepare("SELECT info_hash, creation_date, title, announce_url FROM info WHERE announce_url='http://nyaa.tracker.wf:7777/announce'")?;
        let pull = conn.prepare("SELECT info_hash, creation_date, title, announce_url FROM data_to_track")?;

        let count = conn.query("SELECT COUNT(*) FROM info", &[])?;
        let len : i64= count.iter().nth(0).unwrap().get(0);
        let mut res_vec : Vec<$type> = Vec::with_capacity(len as usize);


        for row in &pull.query(&[])? {
            let hash = row.get(0);
            let date = row.get(1);
            let title = row.get(2);
            let url = row.get(3);
            match $type::new(Some(url), hash, date, title) {
                Ok(data) => res_vec.push(data),
                Err(e) => println!{"serialize to AnnounceComponents error: {:?}", e}
            }
        }

        return Ok(res_vec)
    };
}

pub fn database_announce_components() -> Result<Vec<AnnounceComponents>, Error> {
    construct!(AnnounceComponents);
}
