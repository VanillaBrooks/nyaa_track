


#[derive(Debug)]
pub enum Error{
	IO(std::io::Error),
	Reqwest(reqwest::Error),
	Rss(RssErrors),
	UrlError,
	Torrent(TorrentErrors),
	Announce(AnnounceErrors)
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
	InfoHashFetch(&'static str)
}
#[derive(Debug)]
pub enum TorrentErrors {
	NoAnnounceUrl(String),
	SerdeError(serde_bencode::Error)
}