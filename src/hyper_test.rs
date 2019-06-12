extern crate hyper;
use serde_derive::{Deserialize};
extern crate serde;
extern crate serde_json;

use hyper::Client;
use hyper::rt::{self, Future, Stream};

use super::read::results::ScrapeData;

pub fn run() {
    let url = "http://nyaa.tracker.wf:7777/scrape?info_hash=%25%5c%f8%60%89%e5%c3u%90%90%ac%b6%5e%1b%2d%fd%4a%16%7e%ca".parse::<hyper::Uri>().unwrap();

    let fut = fetch_json(url)
        // use the parsed vector
        .map(|users| {
        
            dbg!{users};
        })
        // if there was an error print it
        .map_err(|e| {
            println!{"there was a general error {:?}", e}
        });

    // Run the runtime with the future trying to fetch, parse and print json.
    //
    // Note that in more complicated use cases, the runtime should probably
    // run on its own, and futures should just be spawned into it.
    // return fut
    rt::run(fut);
}
use super::error::*;
use super::error;
fn fetch_json(url: hyper::Uri) -> impl Future<Item=Result<ScrapeData, Error>, Error=FetchError> {
    let client = Client::new();

    client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let data = body.into_bytes().into_iter().collect::<Vec<_>>();
            Ok(ScrapeData::new_bytes(&data))
        })
}

// Define a type so we can return multiple types of errors
#[derive(Debug)]
enum FetchError {
    Http(hyper::Error),
    // Json(serde_json::Error),
    Error
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

// impl From<serde_json::Error> for FetchError {
//     fn from(err: serde_json::Error) -> FetchError {
//         FetchError::Json(err)
//     }
// }
impl From<error::Error> for FetchError{
    fn from(err: error::Error) -> FetchError {
        FetchError::Error
    }
}