use super::read::{AnnounceComponents, Torrent};
use http;

// from_type: Type that will be converted away from
// to_type: Destination enum that we are converting to
// subtype: the path to the branch of to_type that from_type will be converted to
#[macro_export]
macro_rules! impl_from {
    // catch to expand to a function-like enum
    (empty: $from_type:ident, $to_type:ty, $subtype:expr) => {
        impl_from!(full: $from_type, $to_type, $subtype, exp_empty)
    }; // will expand to call impl_from!(exp_empty: ...)  ^^^^^^^

    // catch for C-like enums
    ($from_type:ident, $to_type:ty, $subtype:expr) => {
        impl_from!(full: $from_type, $to_type, $subtype, exp_full)
    }; // will expand to call impl_from!(exp_full: ...)   ^^^^^^^

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
pub enum Error {
    IO(std::io::Error),
    // Reqwest(reqwest::Error),
    Rss(RssErrors),
    UrlError,
    Torrent(TorrentErrors),
    Announce(AnnounceErrors),
    SliceError(String),
    HTTP(HTTPErrors),
    ShouldNeverHappen(String),
    Futures(FuturesErrors),
    TokioPostgres(tokio_postgres::Error),
}

// impl From<reqwest::Error> for Error {
//     fn from(error: reqwest::Error) -> Error {
//         Error::Reqwest(error)
//     }
// }
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IO(error)
    }
}
impl From<rss::Error> for Error {
    fn from(error: rss::Error) -> Error {
        Error::Rss(RssErrors::RawRssError(error))
    }
}
impl From<serde_bencode::Error> for Error {
    fn from(error: serde_bencode::Error) -> Error {
        Error::Torrent(TorrentErrors::SerdeError(error))
    }
}
impl From<serde_urlencoded::ser::Error> for Error {
    fn from(error: serde_urlencoded::ser::Error) -> Error {
        Error::Announce(AnnounceErrors::SerdeError(error))
    }
}
impl From<tokio_postgres::Error> for Error {
    fn from(error: tokio_postgres::Error) -> Error {
        Error::TokioPostgres(error)
    }
}
impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::HTTP(HTTPErrors::Hyper(err))
    }
}
impl From<http::uri::InvalidUri> for Error {
    fn from(error: http::uri::InvalidUri) -> Error {
        Error::HTTP(HTTPErrors::Uri(error))
    }
}
impl From<futures::channel::mpsc::TrySendError<Torrent>> for Error {
    fn from(_error: futures::channel::mpsc::TrySendError<Torrent>) -> Self {
        Error::Futures(FuturesErrors::TrySendError)
    }
}
impl From<futures::channel::mpsc::TrySendError<AnnounceComponents>> for Error {
    fn from(_error: futures::channel::mpsc::TrySendError<AnnounceComponents>) -> Self {
        Error::Futures(FuturesErrors::TrySendError)
    }
}
impl From<futures::channel::mpsc::SendError> for Error {
    fn from(_error: futures::channel::mpsc::SendError) -> Self {
        Error::Futures(FuturesErrors::SendError)
    }
}

#[derive(Debug)]
pub enum HTTPErrors {
    Hyper(hyper::Error),
    Uri(http::uri::InvalidUri),
    ParseError,
    InvalidData,
}
#[derive(Debug)]
pub enum AnnounceErrors {
    SerdeError(serde_urlencoded::ser::Error),
    AnnounceUrlNone,
    AnnounceUrlError(String),
    AnnounceNotReady(i64),
}

#[derive(Debug)]
pub enum RssErrors {
    RawRssError(rss::Error),
    InfoHashFetch(&'static str),
    RssUrlInvalid,
    CouldNotReadRss,
}
#[derive(Debug)]
pub enum TorrentErrors {
    NoAnnounceUrl(String),
    SerdeError(serde_bencode::Error),
    MissingName,
    InfoHash,
}
#[derive(Debug)]
pub enum FuturesErrors {
    TrySendError,
    SendError,
}
