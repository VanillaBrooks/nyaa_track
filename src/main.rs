mod read_torrent;
use read_torrent::Torrent;

pub mod requests;
pub mod error;
pub mod utils;

#[macro_use]
extern crate lazy_static;

use requests::rss_parse::{self, get_xml, AnnounceComponents};
use requests::url_encoding::{Url, hex_to_char};
// use hashbrown::HashMap;


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


use serde_urlencoded::ser;
fn main() {
	// // nyaa rss feed
	let nyaa_rss_feed = "https://nyaa.si/?page=rss";
	
	
	// // reading stored torrent files
	let mut torrent_files = r"C:\Users\Brooks\Downloads\torrent files\".to_string();
	let mut downloads = r"C:\Users\Brooks\Downloads\".to_string();
	let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
	
	// let mut k = get_xml(nyaa_rss_feed).unwrap();
	// requests::tracking::announce_components(&mut k);

	let dir = r"C:\Users\Brooks\github\nyaa_tracker\torrents";
	// let dir : Vec<std::path::PathBuf> = std::fs::read_dir(dir).unwrap().map(|X| X.unwrap().path()).collect();
	// dbg!{dir};
	utils::serialize_all_torrents(dir);


}



// chars are 67
// quoting 67 to g result: g
// g
// chars are 9b
// g%9b
// chars are ff
// g%9b%ff
// chars are 20
// quoting 20 to   result: %20
// g%9b%ff%20
// chars are 26
// quoting 26 to & result: %26
// g%9b%ff%20%26
// chars are b8
// g%9b%ff%20%26%b8
// chars are a7
// g%9b%ff%20%26%b8%a7
// chars are 6a
// g%9b%ff%20%26%b8%a7%6a
// chars are ce
// g%9b%ff%20%26%b8%a7%6a%ce
// chars are 66
// quoting 66 to f result: f
// g%9b%ff%20%26%b8%a7%6a%cef
// chars are cf
// g%9b%ff%20%26%b8%a7%6a%cef%cf
// chars are fd
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd
// chars are 08
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08
// chars are cb
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb
// chars are c2
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb%c2
// chars are 7d
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb%c2%7d
// chars are 89
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb%c2%7d%89
// chars are c0
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb%c2%7d%89%c0
// chars are b8
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb%c2%7d%89%c0%b8
// chars are 64
// quoting 64 to d result: d
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb%c2%7d%89%c0%b8d
// g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb%c2%7d%89%c0%b8d

// chars are 67
// hex encoding found for character g
// no escape character found for g
//   "g"
// chars are 9b
//   "g%9b"
// chars are ff
//   "g%9b%ff"
// chars are 20
// hex encoding found for character
// escape character found %20
//   "g%9b%ff%20"
// chars are 26
// hex encoding found for character &
// escape character found %26
//   "g%9b%ff%20%26"
// chars are b8
//   "g%9b%ff%20%26%b8"
// chars are a7
//   "g%9b%ff%20%26%b8%a7"
// chars are 6a
// hex encoding found for character j
// no escape character found for j
//   "g%9b%ff%20%26%b8%a7j"
// chars are ce
//   "g%9b%ff%20%26%b8%a7j%ce"
// chars are 66
// hex encoding found for character f
// no escape character found for f
//   "g%9b%ff%20%26%b8%a7j%cef"
// chars are cf
//   "g%9b%ff%20%26%b8%a7j%cef%cf"
// chars are fd
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd"
// chars are 08
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd%08"
// chars are cb
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd%08%cb"
// chars are c2
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd%08%cb%c2"
// chars are 7d
// hex encoding found for character }
// escape character found %7d
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd%08%cb%c2%7d"
// chars are 89
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd%08%cb%c2%7d%89"
// chars are c0
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd%08%cb%c2%7d%89%c0"
// chars are b8
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd%08%cb%c2%7d%89%c0%b8"
// chars are 64
// hex encoding found for character d
// no escape character found for d
//   "g%9b%ff%20%26%b8%a7j%cef%cf%fd%08%cb%c2%7d%89%c0%b8d"

//python
// http://nyaa.tracker.wf:7777/announce?info_hash=d%ef%e5%81%06%aa%8bGc%24%e4%4f%fb%90%a2%00q%e7%fc%d0&peer_id=d%ef%e5%81%06%aa%8bGc%24%e4%4f%fb%90%a2%00q%e7%fc%d0&port=9932&uploaded=0&downloaded=0&numwant=20&compact=1
// http://nyaa.tracker.wf:7777/announce?info_hash=d%ef%e5%81%06%aa%8bgc%24%e4%4f%fb%90%a2%00q%e7%fc%d0&peer_id=d%ef%e5%81%06%aa%8bgc%24%e4%4f%fb%90%a2%00q%e7%fc%d0&port=9932&uploaded=0&downloaded=0&numwant=20&compact=1
// space%20 	%20
// ! 	%21 	%21
// " 	%22 	%22
// # 	%23 	%23
// $ 	%24 	%24
// % 	%25 	%25
// & 	%26 	%26
// ' 	%27 	%27
// ( 	%28 	%28
// ) 	%29 	%29
// * 	%2A 	%2A
// + 	%2B 	%2B
// , 	%2C 	%2C
// - 	%2D 	%2D
// . 	%2E 	%2E
// / 	%2F 	%2F
// 0 	%30 	%30
// 1 	%31 	%31
// 2 	%32 	%32
// 3 	%33 	%33
// 4 	%34 	%34
// 5 	%35 	%35
// 6 	%36 	%36
// 7 	%37 	%37
// 8 	%38 	%38
// 9 	%39 	%39
// : 	%3A 	%3A
// ; 	%3B 	%3B
// < 	%3C 	%3C
// = 	%3D 	%3D
// > 	%3E 	%3E
// ? 	%3F 	%3F
// @ 	%40 	%40
// A 	%41 	%41
// B 	%42 	%42
// C 	%43 	%43
// D 	%44 	%44
// E 	%45 	%45
// F 	%46 	%46
// G 	%47 	%47
// H 	%48 	%48
// I 	%49 	%49
// J 	%4A 	%4A
// K 	%4B 	%4B
// L 	%4C 	%4C
// M 	%4D 	%4D
// N 	%4E 	%4E
// O 	%4F 	%4F
// P 	%50 	%50
// Q 	%51 	%51
// R 	%52 	%52
// S 	%53 	%53
// T 	%54 	%54
// U 	%55 	%55
// V 	%56 	%56
// W 	%57 	%57
// X 	%58 	%58
// Y 	%59 	%59
// Z 	%5A 	%5A
// [ 	%5B 	%5B
// \ 	%5C 	%5C
// ] 	%5D 	%5D
// ^ 	%5E 	%5E
// _ 	%5F 	%5F
// ` 	%60 	%60
// a 	%61 	%61
// b 	%62 	%62
// c 	%63 	%63
// d 	%64 	%64
// e 	%65 	%65
// f 	%66 	%66
// g 	%67 	%67
// h 	%68 	%68
// i 	%69 	%69
// j 	%6A 	%6A
// k 	%6B 	%6B
// l 	%6C 	%6C
// m 	%6D 	%6D
// n 	%6E 	%6E
// o 	%6F 	%6F
// p 	%70 	%70
// q 	%71 	%71
// r 	%72 	%72
// s 	%73 	%73
// t 	%74 	%74
// u 	%75 	%75
// v 	%76 	%76
// w 	%77 	%77
// x 	%78 	%78
// y 	%79 	%79
// z 	%7A 	%7A
// { 	%7B 	%7B
// | 	%7C 	%7C
// } 	%7D 	%7D
// ~ 	%7E 	%7E
//   	%7F 	%7F
// ` 	%80 	%E2%82%AC
//  	%81 	%81
// ‚ 	%82 	%E2%80%9A