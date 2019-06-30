use postgres;
use http;
use super::read::Torrent;

// from_type: Type that will be converted away from
// to_type: Destination enum that we are converting to
// subtype: the path to the branch of to_type that from_type will be converted to
#[macro_export]
macro_rules! impl_from {
    // catch to expand to a function-like enum
    (empty: $from_type:ident, $to_type:ty, $subtype:expr) => {
        impl_from!(full: $from_type, $to_type, $subtype, exp_empty)
    };// will expand to call impl_from!(exp_empty: ...)  ^^^^^^^

    // catch for C-like enums
    ($from_type:ident, $to_type:ty, $subtype:expr) => {
        impl_from!(full: $from_type, $to_type, $subtype, exp_full)
    };// will expand to call impl_from!(exp_full: ...)   ^^^^^^^

    // this branch will be called always. interior macro will expand 
    // depending on if the implementing type expects a C-link enum or not
    (full: $from_type:ident, $to_type:ty, $subtype:expr, $expansion:ident) => {
        impl From<$from_type> for $to_type {
            impl_from!($expansion: $from_type, $subtype);
        }
    };

    //interior expansion function if it is a C-like enum
    (exp_empty: $from_type:ident, $subtype:expr) => {
        fn from(error: $from_type) -> Self {
            $subtype
        }
    };
    // interior expansion function if it is a function-link enum
    (exp_full: $from_type:ident, $subtype:expr) => {
        fn from(error: $from_type) -> Self {
            $subtype(error)
        }
    };
}


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
	Futures(FuturesErrors)
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
impl From<futures::sync::mpsc::TrySendError<Torrent>> for Error {
	fn from(erorr: futures::sync::mpsc::TrySendError<Torrent>) -> Self {
		Error::Futures(FuturesErrors::TrySendError)
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
#[derive(Debug)]
pub enum FuturesErrors {
	TrySendError
}
// use futures::{Async, Poll};
// impl futures::future::Future for Error {
// 	type Item = Error;
// 	type Error = usize;
// 	    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

//             //  Ok(Async::Ready(2 * 2))
// 			Ok(Async::Ready(Error::FuturesError))

//     }
// }
