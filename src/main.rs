mod read_torrent;
use read_torrent::Torrent;

pub mod requests;

#[macro_use]
extern crate lazy_static;

use requests::rss_parse::{self, get_xml};
use requests::url_encoding::{Url, hex_to_char};
// use hashbrown::HashMap;
use std::collections::HashMap;

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
	// let nyaa_rss_feed = "https://nyaa.si/?page=rss";
	
	
	// // reading stored torrent files
	// let mut torrent_files = r"C:\Users\Brooks\Downloads\torrent files\".to_string();
	// let mut downloads = r"C:\Users\Brooks\Downloads\".to_string();
	// let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();

	// git_torrents.push_str("1145877.torrent");
	// let file = &git_torrents;
	// dbg!{Torrent::new_file(file)};
	
	// downloading torrents from internet
	// let torrent_url = r"https://nyaa.si/download/1145877.torrent";

	// let res = rss_parse::download_torrent(Some(torrent_url));
	// println!{"nyaa response: "}
	// dbg!{res};

	// hex_to_char();

	// 1.to_string().as_str()

	// get_xml("Test");

	// let url_struct = Url::new("026a8f0bc3194dbbe545ffd2409ea9cc1f6b7776".to_string(), "026a8f0bc3194dbbe545ffd2409ea9cc1f6b7776".to_string());
	// let k = serde_urlencoded::to_string(url_struct);
	// dbg!{k};
	
	let ann_url = Some("http://nyaa.tracker.wf:7777/announce".to_string());
	let hash = "9a511c42ec9683672f717a404fd62c1ad2e2710d";

	let mut components_struct = rss_parse::AnnounceComponents::new(ann_url, hash).unwrap();
	components_struct.announce();



	// let result = "%01%87%aa%8b%9a%b3%efQ%af%d1s%7a%4b%e4%9e%3e%c1q%1c%b0";
	// let raw = "0187aa8b9ab3ef51afd1737a4be49e3ec1711cb0";

	// assert_eq!(result.to_ascii_lowercase(), hex_to_char(raw).to_ascii_lowercase());

}
//python
//http://nyaa.tracker.wf:7777/announce?info_hash=%3d%96%df%a6%7e%7d%aa%c8%e9Q%a0%e4S%82%025%be%cc%e49&peer_id=%3d%96%df%a6%7e%7d%aa%c8%e9Q%a0%e4S%82%025%be%cc%e49&port=9932&uploaded=0&downloaded=0&numwant=20&compact=1
//http://nyaa.tracker.wf:7777/announce?info_hash=%259c%2509%2505%2501%25bd%256eh%25bf%2501%25e93%25a8%2588s%25b2%25d5%25fc%2501%25ac%25c6&peer_id=%259c%2509%2505%2501%25bd%256eh%25bf%2501%25e93%25a8%2588s%25b2%25d5%25fc%2501%25ac%25c6&port=9973&uploaded=0&downloaded=0&numwant=0&compact=1


// c71783aa9ad4f5f2140f52705882fc33259b1b79

// chars are c7
// %c7
// chars are 17
// %c7%17
// chars are 83
// %c7%17%83
// chars are aa
// %c7%17%83%aa
// chars are 9a
// %c7%17%83%aa%9a
// chars are d4
// %c7%17%83%aa%9a%d4
// chars are f5
// %c7%17%83%aa%9a%d4%f5
// chars are f2
// %c7%17%83%aa%9a%d4%f5%f2
// chars are 14
// %c7%17%83%aa%9a%d4%f5%f2%14
// chars are 0f
// %c7%17%83%aa%9a%d4%f5%f2%14%0f
// chars are 52
// quoting 52 to R result: R
// %c7%17%83%aa%9a%d4%f5%f2%14%0fR
// chars are 70
// quoting 70 to p result: p
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRp
// chars are 58
// quoting 58 to X result: X
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX
// chars are 82
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82
// chars are fc
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82%fc
// chars are 33
// quoting 33 to 3 result: 3
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82%fc3
// chars are 25
// quoting 25 to % result: %25
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82%fc3%25
// chars are 9b
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82%fc3%25%9b
// chars are 1b
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82%fc3%25%9b%1b
// chars are 79
// quoting 79 to y result: y
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82%fc3%25%9b%1by
// %c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82%fc3%25%9b%1by

// space 	%20 	%20
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