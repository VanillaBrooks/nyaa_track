use futures::{Future, Stream};
use tokio_postgres::NoTls;
use tokio;

pub fn run() -> () {


    let fut =
        // Connect to the database
        tokio_postgres::connect("host=localhost user=postgres password=pass", NoTls)

        .map(|(client, connection)| {
            // The connection object performs the actual communication with the database,
            // so spawn it off to run on its own.
            let connection = connection.map_err(|e| eprintln!("connection error: {}", e));
            tokio::spawn(connection);

            // The client is what you use to make requests.
            client
        })

        .and_then(|mut client| {
            // Now we can prepare a simple statement that just returns its parameter.
            client.prepare("SELECT $1::TEXT")
                .map(|statement| (client, statement))
        })

        .and_then(|(mut client, statement)| {
            // And then execute it, returning a Stream of Rows which we collect into a Vec
            client.query(&statement, &[&"hello world"]).collect()
        })

        // Now we can check that we got back the same string we sent over.
        .map(|rows| {
            let value: &str = rows[0].get(0);
            println!{"we made it"}
            assert_eq!(value, "hello world");
        })

        // And report any errors that happened.
        .map_err(|e| {
            eprintln!("error: {}", e);
        });

    // let k:i32 = fut;

    // By default, tokio_postgres uses the tokio crate as its runtime.
    tokio::run(fut);
}