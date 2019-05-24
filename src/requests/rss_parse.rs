use reqwest;

use rss;
use std::fs;
use std::io::prelude::*;

use crate::read_torrent::Torrent;

#[derive(Debug)]
pub enum Error{
	IO(std::io::Error),
	Reqwest(reqwest::Error),
	rss(rss::Error),
	UrlError,
	torrent(serde_bencode::Error)
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
	fn from(error:rss::Error) -> Error{
		return Error::rss(error)
	}
}
impl From<serde_bencode::Error> for Error {
	fn from(error: serde_bencode::Error) ->Error {
		return Error::torrent(error)
	}
}

pub fn get_xml(url: &str) -> Result<(), Error> {

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
		break;

	}
	dbg!{all_data};
	return Ok(())
}



// TODO: configure client pooling
pub fn download_torrent(url: Option<&str>) -> Result<Torrent, Error> {
	dbg!{url};
	if url.is_some(){
		let raw_url = url.unwrap();
		let mut buffer: Vec<u8> = Vec::with_capacity(10_000);
		let k = reqwest::get(raw_url)?.read_to_end(&mut buffer)?;
		
		write_torrent_to_file(&raw_url, &buffer);
		let t = Torrent::new_bytes(&buffer);
	
		Ok(t?)
	}
	else{
		Err(Error::UrlError)
	}
	
}


pub fn write_torrent_to_file(url: &str, data: &Vec<u8>) -> String {
	let mut file_path: String = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
	
	let mut last = 0;
	println!{"here"}
	for i in 0..url.len()-1 {
		if url.get(i..i+1).unwrap() == "/" {
			last = i;
		}
	}

	let filename = &url.get(last+1..url.len()).unwrap();
	file_path.push_str(&filename);
	
	dbg!{&file_path};

	let mut file = std::fs::File::create(&file_path).unwrap();
	file.write_all(&data);


	return file_path
}



pub fn compare_files(f1: &str, f2: &str) -> () {

    let mut buffer1 = Vec::new();
    let mut file1 = std::fs::File::open(f1).unwrap();
    file1.read_to_end(&mut buffer1);

    let mut buffer2 = Vec::new();
    let mut file2 = std::fs::File::open(f2).unwrap();
    file2.read_to_end(&mut buffer2);


    println!{"f1 len:\t{}\tf2 len:\t{}",buffer1.len(), buffer2.len()}

    for i in 0..1000 {
        let c1 = &buffer1[i];
        let c2 = &buffer2[i];
        if c1 == c2 {
            continue
        }
        else{
            println!{"{} {} {}", i, buffer1[i], buffer2[i]}

        }


    }


}