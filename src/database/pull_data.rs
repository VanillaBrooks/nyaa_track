use super::connection;
use super::super::error::*;
use super::super::read::announce_components::AnnounceComponents;

pub fn database_announce_components() -> Result<Vec<AnnounceComponents>, Error> {
    let conn = connection::start()?;
    let pull = conn.prepare("SELECT info_hash, creation_date, title, announce_url FROM info")?;


    let count = conn.query("SELECT COUNT(*) FROM info", &[])?;
    let len : i64= count.iter().nth(0).unwrap().get(0);
    let mut res_vec : Vec<AnnounceComponents> = Vec::with_capacity(len as usize);


    for row in &pull.query(&[])? {
        let hash = row.get(0);
        let date = row.get(1);
        let title = row.get(2);
        let url = row.get(3);
        match AnnounceComponents::new(Some(url), hash, date, title) {
            Ok(data) => res_vec.push(data),
            Err(error) => () //TODO: log the error
        }
    }
    return Ok(res_vec)
}