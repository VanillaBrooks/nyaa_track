use postgres::{Connection, TlsMode};

use futures::sync::mpsc;
use futures::Future;
use futures::Stream;
use tokio_postgres::NoTls;

use super::super::read::*;

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

// url format
//postgresql://postgres:pass@localhost[:port][/database][?param1=val1[[&param2=val2]...]]
const DB_ACCESS : &str= "postgresql://postgres:pass@localhost/nyaa";

pub fn start_sync() -> Result<Connection, postgres::Error>{
    let url : &'static str = DB_ACCESS;
    Connection::connect(url, TlsMode::None)
}


pub fn start_async(rx: mpsc::Receiver<GenericData>) {
    
    let database =
        tokio_postgres::connect(DB_ACCESS, NoTls)

            .map(|(client, connection)| {
                // The connection object performs the actual communication with the database,
                // so spawn it off to run on its own.
                let connection = connection.map_err(|e| eprintln!("connection error: {}", e));
                tokio::spawn(connection);

                // The client is what you use to make requests.
                client
            })

            .and_then(|mut client| {

                client.prepare("INSERT INTO info (info_hash, announce_url, creation_date, title) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING").map(|smt| (client, smt))
            })
            .and_then(|(mut client, prep_info)| {

                client.prepare("with ref_id as (select id from info where info_hash=$1 and announce_url =$2) insert into stats (stats_id, downloaded, seeding, incomplete, poll_time) values ((select * from ref_id), $3,$4,$5,$6)").map(|smt| (client, prep_info, smt))
                // Ok((client, prepare_info, prepare_data))
            })

            .and_then(move |(mut client, prep_info, prep_data)| {

                let data = 
                    rx.for_each(move |res|{

                        // println!{"database write"};

                        // get pointer references to interior of Arc
                        raw!{into;
                            res.hash => hash,
                            res.url => url,
                            res.creation_date => creation_date,
                            res.title => title
                        }

                        unsafe{
                            client.query(&prep_info, &[&*hash, &*url, &*creation_date, &*title]).collect().poll();
                            client.query(&prep_data, &[&*hash, &*url, &res.downloaded, &res.complete, &res.incomplete, &res.poll_time]).collect().poll();
                        }

                        // move back to Arc to prevent memory leak
                        raw!{from; hash, url, creation_date, title}

                        Ok(())
                    })
                    .map(|_| println!{"finished insertion"});
                tokio::spawn(data);


            Ok(())

            });

    let fut = 
        database
            // Now we can check that we got back the same string we sent over.
            .map(|res| {
                println!{"the database has been dropped {:?}", res}
            })
            // And report any errors that happened.
            .map_err(|e| {
                eprintln!("database future error: {} :: ", e);
            });

    tokio::spawn(fut);

}
