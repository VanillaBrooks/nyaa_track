use regex;
use hashbrown;

use encoding::bencode::{Bencode};
#[macro_use]
mod encoding;



fn main() {
	// let k :i32= regex::Regex::new("234").unwrap();

	// let sample = ":spami42eed3:bar4:spam3:fooi42ee";
	let digit = "i-42e";
	let list = "l4:spami42ee";
	let dict = "d3:bar4:spam3:fooi42ee";
	let strin = "4:spam";
	// let a = vec![
	// get_next_stop(&digit),
	// get_next_stop(&list),
	// get_next_stop(&dict),
	// get_next_stop(&strin),
	// ];
	// for i in a{
	// 	dbg!{i};
	// }
	dbg!{bencode![[1,2,3,4,6]]};

	    if true || true {
        println!{"here"}
    }
	
	// encoding::bencode::dic("dsample");
}

