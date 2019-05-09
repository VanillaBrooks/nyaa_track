use regex;
use hashbrown;

use encoding::bencode::{Bencode};
#[macro_use]
mod encoding;
use regex::Regex;
use std::io;
use std::fs::File;
use std::io::Read;

fn read_file(name: &str) -> Result<Vec<u8>, io::Error> {
	let fname =format!("C:\\Users\\Brooks\\Downloads\\{}", name);
	let mut f = File::open(&fname)?;
    let mut buffer : Vec<u8> = Vec::with_capacity(100);
	// let mut buffer = [0; 50];
    f.read(&mut buffer)?;

	dbg!{&buffer};

    // read into a String, so that you don't need to do the conversion.
    // let mut buffer = String::new();
    // f.read_to_string(&mut buffer)?;

	let contents = std::fs::read_to_string(&fname);
	dbg!{contents};

	// return Ok(vec![4])
	return Ok(buffer)
}


fn main() {

	let dict = "d3:bar4:spam3:fooi42ei-42el4:spami42eei34ee";

	dbg!{read_file("onejav.com_docp145_2.torrent")};
	dbg!{read_file("test.txt")};

	encoding::bencode::general(&dict);


}

