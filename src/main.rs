use regex;
use hashbrown;

use encoding::bencode::{Bencode};
#[macro_use]
mod encoding;
use regex::Regex;



fn main() {
	// let k :i32= regex::Regex::new("234").unwrap();

	// let sample = ":spami42eed3:bar4:spam3:fooi42ee";
	// let digit = "i-42e";
	// let list = "l4:spami42ee";
	let dict = "d3:bar4:spam3:fooi42ei-42el4:spami42eei34ee";
	// let strin = "4:spam";
	// let num = "l4:spami-42ee";

	let lis_start = Regex::new(r"l[idl\d]").unwrap();
	let lis_end = Regex::new(r"ee|\d:[[:alpha:]]+e^[i\dd]").unwrap();
	let dict_end = Regex::new(r"d[l\di]").unwrap();

	let starts: Vec<_> = lis_start.find_iter(&dict).map(|x| x.start()+1).collect();
	let ends :Vec<_>= lis_end.find_iter(&dict).map(|x| x.end()-1).collect();
	let dict_ends : Vec<_> = dict_end.find_iter(&dict).map(|x| x.end()-1).collect();

	// println!{"dict starts: {:?} list starts: {:?} ends: {:?}", dict_ends, starts, ends}

	// encoding::bencode::general(&dict);
	// println!{"{}", dict.get(1..42).unwrap()}
	// dbg!{dict.get(27..37)};

	encoding::bencode::find_all_strings(&dict);


}

