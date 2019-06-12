use postgres;
use http;

#[derive(Debug)]
pub enum Error{
	IO(std::io::Error),
	Reqwest(reqwest::Error),
	Rss(RssErrors),
	UrlError,
	Torrent(TorrentErrors),
	Announce(AnnounceErrors),
	SliceError(String),
	Postgres(postgres::error::Error),
	HTTP(HTTPErrors),
	ShouldNeverHappen(String),
	FuturesError
}

impl From<reqwest::Error> for Error{
	fn from(error: reqwest::Error) -> Error{
		return Error::Reqwest(error)
	}
}
impl From<std::io::Error> for Error{
	fn from(error: std::io::Error) -> Error{
		return Error::IO(error)
	}
}
impl From<rss::Error> for Error {
	fn from(error: rss::Error) -> Error{
		return Error::Rss(RssErrors::RawRssError(error))
	}
}
impl From<serde_bencode::Error> for Error {
	fn from(error: serde_bencode::Error) ->Error {
		return Error::Torrent(TorrentErrors::SerdeError(error))
	}
}
impl From<serde_urlencoded::ser::Error> for Error {
	fn from(error: serde_urlencoded::ser::Error) -> Error {
		return Error::Announce(AnnounceErrors::SerdeError(error))
	}
}
impl From<postgres::error::Error> for Error {
	fn from(error: postgres::error::Error) -> Error {
		return Error::Postgres(error)
	}
}
impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::HTTP(HTTPErrors::Hyper(err))
    }
}
impl From<http::uri::InvalidUri> for Error {
	fn from(error: http::uri::InvalidUri) -> Error {
		return Error::HTTP(HTTPErrors::Uri(error))
	}

}
#[derive(Debug)]
pub enum HTTPErrors {
	Hyper(hyper::Error),
	Uri(http::uri::InvalidUri)
}
#[derive(Debug)]
pub enum AnnounceErrors{
	SerdeError(serde_urlencoded::ser::Error),
	AnnounceUrlNone,
	AnnounceUrlError(String),
	AnnounceNotReady(i64)
}

#[derive(Debug)]
pub enum RssErrors {
	RawRssError(rss::Error),
	InfoHashFetch(&'static str),
	RssUrlInvalid,
	CouldNotReadRss
}
#[derive(Debug)]
pub enum TorrentErrors {
	NoAnnounceUrl(String),
	SerdeError(serde_bencode::Error),
	MissingName
}
use futures::{Async, Poll};
impl futures::future::Future for Error {
	type Item = Error;
	type Error = usize;
	    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

            //  Ok(Async::Ready(2 * 2))
			Ok(Async::Ready(Error::FuturesError))

    }
}