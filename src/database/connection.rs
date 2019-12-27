use postgres::{tls, Client};

use futures::channel::mpsc;
use futures::Future;
use futures::Stream;
use futures::StreamExt;
use tokio_postgres::NoTls;

use super::super::read::*;

use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use std::sync::Arc;

macro_rules! raw {
    (into; $($arc_name:expr => $new_name:ident),+) => {
        $(
            // dbg!{"hi"};
            let $new_name = Arc::into_raw($arc_name);
        )+
    };
    (from; $($arc_name:ident),+) => {
        $(
            #[allow(unused_variables)]
            let $arc_name = unsafe {Arc::from_raw($arc_name)};
        )+
    };
}

#[derive(Serialize, Deserialize)]
struct DatabaseConfig {
    port: u32,
    database_name: String,
    username: String,
    password: String,
}

impl DatabaseConfig {
    fn new() -> Self {
        let path = r".\config.json".to_string();

        let file = fs::File::open(path).expect("config.json DOES NOT EXIST");
        let reader = io::BufReader::new(file);

        serde_json::de::from_reader(reader)
            .expect("port, database, username, password were not all filled.")
    }
    fn connection_url(&self) -> String {
        format! {"postgresql://{}:{}@localhost:{}/{}",self.username, self.password, self.port, self.database_name}
    }
}

// url format
//postgresql://postgres:pass@localhost[:port][/database][?param1=val1[[&param2=val2]...]]
const DB_ACCESS: &str = "postgresql://postgres:pass@localhost/nyaa";

pub fn start_sync() -> Result<Client, postgres::Error> {
    let url: &'static str = DB_ACCESS;
    let url: String = DatabaseConfig::new().connection_url();
    Client::connect(&url, tls::NoTls)
}

pub fn start_async(rx: mpsc::Receiver<DatabaseUpsert>) {
    let db_url = DatabaseConfig::new().connection_url();

    let database =
        tokio_postgres::connect(&db_url, NoTls)

            .map(|(client, connection)| {
                // The connection object performs the actual communication with the database,
                // so spawn it off to run on its own.
                let connection = connection.map_err(|e| eprintln!("connection error: {}", e));
                tokio::spawn(connection);

                // The client is what you use to make requests.
                client
            })
                // Prepare all the statements we use to execute
                // AFAIK there is not a cleaner way to do this (besides macros)
            .and_then(|mut client| {
                client.prepare("INSERT INTO info (info_hash, announce_url, creation_date, title) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING").map(|x| (client, x))
            })
            .and_then(|(mut client, prep_info)|{
                client.prepare("with ref_id as (select id from info where info_hash=$1 and announce_url =$2) insert into stats (stats_id, downloaded, seeding, incomplete, poll_time) values ((select * from ref_id), $3,$4,$5,$6)").map(|x| (client, prep_info, x))
            })
            .and_then(|(mut client, prep_info, prep_data)| {
                client.prepare("with type_id_ as ( select type_id from error_types where error_name = $1 ), info_id_ as ( select id from info where info_hash = $2 ) insert into error (err_type, info_id, poll_time) VALUES ( (select * from type_id_), (select * from info_id_), $3);").map(|prep_err| (client, prep_info, prep_data, prep_err))
            })
            .and_then(move |(mut client, prep_info, prep_data, prep_err)| {

                let data = 
                    rx.for_each(move |upsert_enum|{

                        match upsert_enum {
                            DatabaseUpsert::Data(res) => {
                                dbg!{"database write!"};
                                // get pointer references to interior of Arc
                                raw!{into;
                                    res.hash => hash,
                                    res.url => url,
                                    res.title => title
                                }

                                unsafe{
                                    client.query(&prep_info, &[&*hash, &*url, &res.creation_date, &*title]).collect().poll();
                                    client.query(&prep_data, &[&*hash, &*url, &res.downloaded, &res.complete, &res.incomplete, &res.poll_time]).collect().poll();
                                }

                                // move back to Arc to prevent memory leak
                                raw!{from; hash, url, title}
                            }

                            DatabaseUpsert::Error((hash, err, poll_time)) =>{
                                raw!{into;
                                    hash => info_hash_ptr
                                }

                                unsafe{
                                    client.query(&prep_err, &[&err.to_str(), &*info_hash_ptr, &poll_time]).collect().poll();
                                }

                                raw!{from; info_hash_ptr}
                                
                            } // error match 
                        }// total match


                        Ok(())
                    })
                    .map(|_| println!{"finished insertion"});
                tokio::spawn(data);


            Ok(())

            });

    let fut = database
        // Now we can check that we got back the same string we sent over.
        .map(|res| {
            println! {"the database has been dropped {:?}", res}
        })
        // And report any errors that happened.
        .map_err(|e| {
            eprintln!("database future error: {} :: ", e);
        });

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
    pub fn new(x: ErrorType, hash: Arc<String>, poll_time: i64) -> DatabaseUpsert {
        match x {
            ErrorType::InvalidAnnounce => DatabaseUpsert::Error((hash, x, poll_time)),
            ErrorType::InvalidScrape => DatabaseUpsert::Error((hash, x, poll_time)),
        }
    }

    fn to_str(&self) -> &'a str {
        match self {
            InvalidAnnounce => "invalid announce",
            InvalidScrape => "invalid scrape",
        }
    }
}
