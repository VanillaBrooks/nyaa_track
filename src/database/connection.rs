use postgres::{Connection, TlsMode};

use futures::sync::mpsc;
use futures::Future;
use futures::Stream;
use tokio_postgres::NoTls;

use super::super::read::*;

//postgresql://postgres:pass@localhost[:port][/database][?param1=val1[[&param2=val2]...]]
pub fn start_sync() -> Result<Connection, postgres::Error>{
    let url : &'static str = "postgresql://postgres:pass@localhost/nyaa";
    Connection::connect(url, TlsMode::None)
}


pub fn start_async(mut rx: mpsc::Receiver<GenericData>) {
    
    let database =
        tokio_postgres::connect("postgresql://postgres:pass@localhost/nyaa", NoTls)

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
                // loop {
                let data = 
                    rx.for_each(move |res|{
                        dbg!{"database write"};
                        client.query(&prep_info, &[&res.hash, &res.url, &res.creation_date, &res.title]).collect().poll();
                        client.query(&prep_data, &[&res.hash, &res.url, &res.downloaded, &res.complete, &res.incomplete, &res.poll_time]).collect().poll();
                        Ok(())
                    })
                    .map(|x|println!{"finished insertion"});
                tokio::spawn(data);
                // }

            Ok(())
                // let fut = recv;
                    // .map(|x| ());
                    // .map_err(|x| ());
                // tokio::spawn(fut);
                // Ok(())
            });

    let fut = 
        database
            // Now we can check that we got back the same string we sent over.
            .map(|res| {
                println!{"the database has been dropped"}
            })
            // And report any errors that happened.
            .map_err(|e| {
                eprintln!("database future error: {} :: ", e);
            });

    tokio::spawn(fut);

}


	// let prepare_info = conn.prepare("INSERT INTO info (info_hash, announce_url, creation_date, title) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING").unwrap();
	// let prepare_data = conn.prepare("with ref_id as (select id from info where info_hash=$1 and announce_url =$2) insert into stats (stats_id, downloaded, seeding, incomplete, poll_time) values ((select * from ref_id), $3,$4,$5,$6)").unwrap();