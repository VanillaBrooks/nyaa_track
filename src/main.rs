mod read_torrent;
use read_torrent::Torrent;
// mod serde_test;
mod requests;

use requests::rss_parse::{self, get_xml};


fn create_torrent(path: &str) ->() {
	let k = Torrent::new_file(path);

	match k{
		Ok(x) => {
			println!{"all good"}
			dbg!{x};
		} 
		Err(x) => println!{"there was an error:\n{:?}",x}
	}
}


fn main() {
	// nyaa rss feed
	let nyaa_rss_feed = "https://nyaa.si/?page=rss";
	
	
	// reading stored torrent files
	let mut torrent_files = r"C:\Users\Brooks\Downloads\torrent files\".to_string();
	let mut downloads = r"C:\Users\Brooks\Downloads\".to_string();
	let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();

	let file = &git_torrents;
	dbg!{Torrent::new_file(file)};
	
	// downloading torrents from internet
	let torrent_url = r"https://nyaa.si/download/1145877.torrent";

	let res = rss_parse::download_torrent(Some(torrent_url));
	println!{"nyaa response: "}
	dbg!{res};

}

