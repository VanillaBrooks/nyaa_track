
use std::io::prelude::*;
use reqwest;
// mod error;
use super::error::{Error, AnnounceErrors, RssErrors, TorrentErrors };

use std::time::{SystemTime, UNIX_EPOCH};
use super::read_torrent;

use super::read_torrent::{Torrent, Announce};
// TODO: configure client pooling
// probably want to turn this thing into a struct
pub fn download_torrent(url: Option<&str>, save_name: &str) -> Result<Torrent, Error> {
	dbg!{url};
	if url.is_some(){
		let raw_url = url.unwrap();
		let mut buffer: Vec<u8> = Vec::with_capacity(10_000);
		let k = reqwest::get(raw_url)?.read_to_end(&mut buffer)?;
		
		write_torrent_to_file(&raw_url, &buffer, &save_name);
		let t = Torrent::new_bytes(&buffer);
	
		Ok(t?)
	}
	else{
		Err(Error::UrlError)
	}
	
}

// generate a .torrent file for the data
pub fn write_torrent_to_file(url: &str, data: &Vec<u8>, save_name: &str) -> String {
    let mut base = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
    base.push_str(save_name);
    base.push_str(".torrent");

	let mut file = std::fs::File::create(&base).unwrap();
	file.write_all(&data);

    return base
}


// BASE SAVE PATH
pub fn content_after_last_slash<'a> (url: &'a str) -> Result<&'a str, Error> {
    let mut file_path: String = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
	
	let mut last = 0;
	for i in 0..url.len()-1 {
		if url.get(i..i+1).unwrap() == "/" {
			last = i;
		}
	}

	match url.get(last+1..url.len()) {
        Some(slice) => Ok(slice),
        None => Err(Error::SliceError("did not contain a slash. you fucked up somewhere".to_string()))
    }
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
    if buffer1.len()  > f2.len() {
        len = buffer2.len()
    }
    else{
        len = buffer1.len()
    }

    for i in 0..(len-1 ){
        let c1 = &buffer1[i];
        let c2 = &buffer2[i];
        if c1 == c2 {
            println!{"match"}
            continue
        }
        else{
            println!{"{} {} {}", i, buffer1[i], buffer2[i]}
            break
        }
    } //for

    Ok(())
}

pub fn get_unix_time() -> i64 {

    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
}

pub fn serialize_all_torrents(directory: &str) ->  Vec<read_torrent::Torrent>{
    let dir : Vec<_>= std::fs::read_dir(directory)
        .unwrap()
        .map(|x| 
        read_torrent::Torrent::new_file(
                x.unwrap()
                .path()
                .to_str()
                .unwrap()
            )
        )
        .filter(|torrent| torrent.is_ok())
        .map(|ok_torrent| ok_torrent.unwrap())
        .collect();

    return dir;
    
}


pub fn check_hashes(dir_to_read: &str) -> () {//Vec<(String, Torrent)>{

    let dir : Vec<_> = std::fs::read_dir(dir_to_read).unwrap()
        .map(|x| x.unwrap().path())
        .map(|x|{
            let txt_path = x.to_str().unwrap();
            let contents = read_torrent::Torrent::new_file(&txt_path).unwrap();
            (txt_path.to_string(), contents)
        })
        .collect();

    let mut good = 0;
    let mut bad : Vec<String>= Vec::new();

    for (filename, mut torrent) in dir {
        let period = filename.find(".").unwrap();
        let hash = filename.get(45..period).unwrap();
        if hash == torrent.info_hash().unwrap(){
            good+=1;
        }
        else {
            println!{"{}\n{}\n do not match \n\n", hash, torrent.info_hash().unwrap()}
            bad.push(hash.to_string());

        }
    }

    println!{"good hashes:\t {}\tbad hashes:\t {}", good, bad.len()}
    if bad.len() >0{
        dbg!{bad};
    }

    
    // return dir;
}