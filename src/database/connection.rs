use futures::channel::mpsc;
use futures::StreamExt;
use tokio_postgres::NoTls;

use super::super::read::*;
use super::super::traits::WontError;

use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    port: u32,
    database_name: String,
    username: String,
    password: String,
}

impl DatabaseConfig {
    pub(crate) fn new() -> Self {
        let path = r".\config.json".to_string();

        let file = fs::File::open(path).expect("config.json DOES NOT EXIST");
        let reader = io::BufReader::new(file);

        serde_json::de::from_reader(reader)
            .expect("port, database, username, password were not all filled.")
    }
    pub(crate) fn connection_url(&self) -> String {
        format! {"postgresql://{}:{}@localhost:{}/{}",self.username, self.password, self.port, self.database_name}
    }
}

// url format
//postgresql://postgres:pass@localhost[:port][/database][?param1=val1[[&param2=val2]...]]

pub fn start_async(mut rx: mpsc::Receiver<DatabaseUpsert>) {
    let db_url = DatabaseConfig::new().connection_url();

    let fut =async move {
        let (client, _connection) = tokio_postgres::connect(&db_url, NoTls).await.unwrap();
        let prep_info = client.prepare("INSERT INTO info (info_hash, announce_url, creation_date, title) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING").await.unwrap();
        let prep_data = client.prepare("with ref_id as (select id from info where info_hash=$1 and announce_url =$2) insert into stats (stats_id, downloaded, seeding, incomplete, poll_time) values ((select * from ref_id), $3,$4,$5,$6)").await.unwrap();
        let prep_err = client.prepare("with type_id_ as ( select type_id from error_types where error_name = $1 ), info_id_ as ( select id from info where info_hash = $2 ) insert into error (err_type, info_id, poll_time) VALUES ( (select * from type_id_), (select * from info_id_), $3);").await.unwrap();

        while let Some(upsert_enum) = rx.next().await {
            match upsert_enum {
                DatabaseUpsert::Data(res) => {
                    client
                        .query(
                            &prep_info,
                            &[&*res.hash, &*res.url, &res.creation_date, &*res.title],
                        )
                        .await.wont_error(&format!{"line: {}", line!{}});
                    client
                        .query(
                            &prep_data,
                            &[
                                &*res.hash,
                                &*res.url,
                                &res.downloaded,
                                &res.complete,
                                &res.incomplete,
                                &res.poll_time,
                            ],
                        )
                        .await.wont_error(&format!{"line: {}", line!{}});
                }

                DatabaseUpsert::Error((hash, err, poll_time)) => {
                    client
                        .query(&prep_err, &[&err.to_str(), &*hash, &poll_time])
                        .await.wont_error(&format!{"line: {}", line!{}});
                } // error match
            } // total match

            dbg! {"finsihed insertion"};
        }
    };

    tokio::spawn(fut);
}

pub enum DatabaseUpsert {
    Data(GenericData),
    Error((Arc<String>, ErrorType, i64)),
}

pub enum ErrorType {
    InvalidAnnounce,
    InvalidScrape,
}

impl<'a> ErrorType {
    pub fn upsert(errtype: ErrorType, hash: Arc<String>, poll_time: i64) -> DatabaseUpsert {
        match errtype {
            ErrorType::InvalidAnnounce => DatabaseUpsert::Error((hash, errtype, poll_time)),
            ErrorType::InvalidScrape => DatabaseUpsert::Error((hash, errtype, poll_time)),
        }
    }

    fn to_str(&self) -> &'a str {
        match self {
            Self::InvalidAnnounce => "invalid announce",
            Self::InvalidScrape => "invalid scrape",
        }
    }
}
