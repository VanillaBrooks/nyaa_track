
use std::io::prelude::*;
use reqwest;
// mod error;
use super::error::{Error, AnnounceErrors, RssErrors, TorrentErrors };

use std::time::{SystemTime, UNIX_EPOCH};
use super::read_torrent;

use super::read_torrent::{Torrent, Announce};
// TODO: configure client pooling
// probably want to turn this thing into a struct
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

// generate a .torrent file for the data
pub fn write_torrent_to_file(url: &str, data: &Vec<u8>) -> String {
	let mut file_path: String = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
	
	let mut last = 0;
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


pub fn compare_files(f1: &str, f2: &str) -> Result<(), Error> {

    let mut buffer1 = Vec::new();
    let mut file1 = std::fs::File::open(f1).unwrap();
    file1.read_to_end(&mut buffer1)?;

    let mut buffer2 = Vec::new();
    let mut file2 = std::fs::File::open(f2).unwrap();
    file2.read_to_end(&mut buffer2)?;


    println!{"f1 len:\t{}\tf2 len:\t{}",buffer1.len(), buffer2.len()}

    let mut len = 0;
    if f1.len()  > f2.len() {
        len = f2.len()
    }
    else{
        len = f1.len()
    }

    for i in 0..len {
        let c1 = &buffer1[i];
        let c2 = &buffer2[i];
        if c1 == c2 {
            continue
        }
        else{
            println!{"{} {} {}", i, buffer1[i], buffer2[i]}
        }
    } //for

    Ok(())
}

pub fn get_unix_time() -> u64 {

    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}

pub fn serialize_all_torrents(directory: &str) -> Vec<read_torrent::Torrent>{

	// let dir : Vec<std::path::PathBuf>
    let dir : Vec<Result<read_torrent::Torrent, _>> = std::fs::read_dir(directory)
                                        .unwrap()
                                        .map(|x| read_torrent::Torrent::new_file(
                                                x.unwrap().path().to_str().unwrap()
                                            )   
                                        )
                                        .collect();
    let mut dir_unwrapped = Vec::with_capacity(dir.len());

    dir.into_iter().for_each(|x| {
        match x{
            Ok(k) => dir_unwrapped.push(k),
            Err(k) => println!{"could not read stored torrent with error {:?}", k}
        }});

    return dir_unwrapped
    
}