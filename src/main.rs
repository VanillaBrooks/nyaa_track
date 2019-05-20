mod read_torrent;
// mod serde_test;
mod requests;

use requests::rss_parse::get_xml;


fn main() {
	// #![cfg(feature = "from_url")]

	let nyaa = "https://nyaa.si/?page=rss";
	// get_rss(nyaa);
	// test(1usize)


	// let path = r"C:\Users\Brooks\github\nyaa_tracker\temp\temp.xml";
	// let k = reqwest::get(nyaa).unwrap().text().unwrap();
	
	get_xml(nyaa);

	

	// dbg!{g};
}

