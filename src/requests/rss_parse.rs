use reqwest;

use rss;
use std::fs;
use std::io::Write;

#[derive(Debug)]
pub enum Error{
	IO,
	Reqwest,
	rss,
	UrlError
}
impl From<reqwest::Error> for Error{
	fn from(T: reqwest::Error) -> Error{
		return Error::Reqwest
	}
}
impl From<std::io::Error> for Error{
	fn from(T:std::io::Error) -> Error{
		return Error::IO
	}
}
impl From<rss::Error> for Error {
	fn from(T:rss::Error) -> Error{
		return Error::rss
	}
}

pub fn get_xml(url: &str) -> Result<(), Error>{

	//TODO: move this to a lazy_static!{}
	let temp_folder : &'static str = r"C:\Users\Brooks\github\nyaa_tracker\temp";
	let tmp = fs::create_dir(temp_folder);	

	let path : &'static str = r"C:\Users\Brooks\github\nyaa_tracker\temp\temp.xml";

	// let mut req = reqwest::get(url)?;
	// let xml_data = reqwest::get(url)?.text()?;


	// let mut file = fs::File::create(path)?;
	// file.write_all(xml_data.as_bytes());
	

	let mut file = fs::File::open(path)?;
	let channel = rss::Channel::read_from(std::io::BufReader::new(file))?;
	let items = channel.items();


	let mut all_data = Vec::with_capacity(items.len());

	for i in items{

		//TODO: better handling for bad requests
		match download_torrent(i.link()){
			Ok(x) => all_data.push(x),
			Err(x) => println!{"there was an error with torrent link: {:?}", x}
		}

	}

	return Ok(())
}



// TODO: Serialize the torrent in this function
fn download_torrent(url: Option<&str>) -> Result<String, Error> {
	if url.is_some(){
		let k = reqwest::get(url.unwrap())?.text()?;
		Ok(k)
	}
	else{
		Err(Error::UrlError)
	}
	
}
