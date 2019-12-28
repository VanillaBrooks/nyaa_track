use super::super::error::*;
use super::super::utils;
use hashbrown::HashMap;
use lazy_static::lazy_static;
// expects string to be lowercase
pub fn hex_to_char(input: &str) -> String {
    // let input = input.to_uppercase();
    let mut input_clone = String::with_capacity(20);

    lazy_static! {
        static ref HEX: HashMap<String, String> = _hex_to_char();
        static ref PERCENT_ENCODE: HashMap<String, String> = reserved_characters();
        static ref INDEXES: Vec<usize> = (0..40).step_by(2).collect();
    }

    for i in INDEXES.iter() {
        let chars = input.get(*i..*i + 2).unwrap();
        // println!{"chars are {}", chars}

        match HEX.get(chars) {
            // the two character combination matches something in HEX
            Some(x) => {
                // println!{"HEX encoding found for character {}", x}
                //get escape character
                match PERCENT_ENCODE.get(x) {
                    // it has an escape character, push the escaped version
                    Some(escape_char) => {
                        // println!{"escape character found {}", escape_char}
                        input_clone.push_str(&escape_char)
                    }
                    // no escaped version, we can push the original
                    None => {
                        // println!{"no escape character found for {}",x};
                        input_clone.push_str(x);
                    }
                }
            }
            // The HEX combination does not mean anything, push %<chars>
            None => {
                // println!{"no HEX conversion for that character"}
                input_clone.push_str("%");
                input_clone.push_str(chars);
            }
        }
        // dbg!{&input_clone};
    }

    input_clone
}
fn _hex_to_char() -> HashMap<String, String> {
    let mut chars: Vec<&str> = "! \" # $ % & ' ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = > ? @ A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \\ ] ^ _ ` a b c d e f g h i j k l m n o p q r s t u v w x y z { | } ~".split_ascii_whitespace().collect();
    let letters = ["A", "B", "C", "D", "E", "F"];
    let mut hex: Vec<String> = Vec::new();
    chars.insert(0, " ");

    for i in 2..8 {
        // let mut new : Vec<String> = Vec::new();

        for j in (i * 10)..((i + 1) * 10) {
            hex.push(j.to_string());
        }
        for character in &letters {
            let mut num = i.to_string();
            num.push_str(character);
            hex.push(num)
        }
    }
    let mut hm: HashMap<String, String> = HashMap::new();

    for i in 0..chars.len() {
        hm.insert(hex[i].clone(), chars[i].to_string().clone());
    }

    hm
}

fn reserved_characters() -> HashMap<String, String> {
    let mut keys: Vec<&str> = "! \" # $ % & ' ( ) * + , / : ; = ? @ [ ] < > - . ^ _ ` { | } ~"
        .split_ascii_whitespace()
        .collect();
    let mut values: Vec<&str> =
        "%20 %21 %22 %23 %24 %25 %26 %27 %28 %29 %2A \
         %2B %2C %2F %3A %3B %3D %3F %40 %5B %5D %3C %3E %2D %2E %5E %5F %60 %7B %7C %7D %7E"
            .split_ascii_whitespace()
            .collect();
    keys.insert(0, " ");

    let mut hm: HashMap<String, String> = HashMap::new();
    for _ in 0..keys.len() {
        hm.insert(
            keys.remove(0).to_string(),
            values.remove(0).to_string().to_ascii_lowercase(),
        );
    }
    hm
}

#[derive(Debug)]
pub struct AnnounceUrl {
    info_hash: String,
    peer_id: String,
    port: u32,
    uploaded: u32,
    downloaded: u32,
    numwant: u32,
    compact: u32,
}

impl AnnounceUrl {
    pub fn new(info_hash: String, peer_id: String) -> AnnounceUrl {
        let url_hash = hex_to_char(&info_hash);
        let peer_id_hash = hex_to_char(&peer_id);
        AnnounceUrl {
            info_hash: url_hash,
            peer_id: peer_id_hash,
            port: 9932,
            uploaded: 0,
            downloaded: 0,
            numwant: 20,
            compact: 1,
        }
    }

    // build the url format that is required
    // ....../announce?info_hash=......&peer_id=......& etc
    pub fn serialize(&self, base_announce: &str) -> String {
        let mut s = String::with_capacity(50);
        s.push_str(&base_announce);
        s.push_str("?");

        Self::_seialze_helper(&mut s, "info_hash", &self.info_hash);
        Self::_seialze_helper(&mut s, "peer_id", &self.peer_id);
        Self::_seialze_helper(&mut s, "port", &self.port.to_string());
        Self::_seialze_helper(&mut s, "uploaded", &self.uploaded.to_string());
        Self::_seialze_helper(&mut s, "downloaded", &self.downloaded.to_string());
        Self::_seialze_helper(&mut s, "numwant", &self.numwant.to_string());
        Self::_seialze_helper(&mut s, "compact", &self.compact.to_string());

        println! {"{}", s}
        s
    }

    fn _seialze_helper(base: &mut String, cat: &str, var: &str) {
        if !base.is_empty() {
            base.push_str("&");
        }

        base.push_str(cat);
        base.push_str("=");
        base.push_str(&var);
    }
}

#[derive(Debug)]
pub struct ScrapeUrl {
    pub hash: String,
}
impl ScrapeUrl {
    pub fn new(hash: &str) -> ScrapeUrl {
        let url_hash = hex_to_char(hash);
        ScrapeUrl { hash: url_hash }
    }
    pub fn announce_to_scrape(&self, ann_url: &str) -> Result<String, Error> {
        let mut base = String::with_capacity(80);
        let no_announce_url = utils::content_before_last_slash(&ann_url)?; //TODO: LOG the announce url we come up with here (could be problematic)
        base.push_str(&no_announce_url);
        base.push_str("scrape?");

        base.push_str("info_hash=");
        base.push_str(&self.hash);

        match base.get(0..base.len() - 1) {
            Some(_) => Ok(base.to_string()),
            None => Err(Error::SliceError(
                "slicing url could not be done. this should not happen".to_string(),
            )), //TODO: Log the error
        }
    }
}

// {'20': ' ',
//  '21': '!',
//  '22': '"',
//  '23': '#',
//  '24': '$',
//  '25': '%',
//  '26': '&',
//  '27': "'",
//  '28': '(',
//  '29': ')',
//  '2A': '*',
//  '2B': '+',
//  '2C': ',',
//  '2D': '-',
//  '2E': '.',
//  '2F': '/',
//  '30': '0',
//  '31': '1',
//  '32': '2',
//  '33': '3',
//  '34': '4',
//  '35': '5',
//  '36': '6',
//  '37': '7',
//  '38': '8',
//  '39': '9',
//  '3A': ':',
//  '3B': ';',
//  '3C': '<',
//  '3D': '=',
//  '3E': '>',
//  '3F': '?',
//  '40': '@',
//  '41': 'A',
//  '42': 'B',
//  '43': 'C',
//  '44': 'D',
//  '45': 'E',
//  '46': 'F',
//  '47': 'G',
//  '48': 'H',
//  '49': 'I',
//  '4A': 'J',
//  '4B': 'K',
//  '4C': 'L',
//  '4D': 'M',
//  '4E': 'N',
//  '4F': 'O',
//  '50': 'P',
//  '51': 'Q',
//  '52': 'R',
//  '53': 'S',
//  '54': 'T',
//  '55': 'U',
//  '56': 'V',
//  '57': 'W',
//  '58': 'X',
//  '59': 'Y',
//  '5A': 'Z',
//  '5B': '[',
//  '5C': '\\',
//  '5D': ']',
//  '5E': '^',
//  '5F': '_',
//  '60': '`',
//  '61': 'a',
//  '62': 'b',
//  '63': 'c',
//  '64': 'd',
//  '65': 'e',
//  '66': 'f',
//  '67': 'g',
//  '68': 'h',
//  '69': 'i',
//  '6A': 'j',
//  '6B': 'k',
//  '6C': 'l',
//  '6D': 'm',
//  '6E': 'n',
//  '6F': 'o',
//  '70': 'p',
//  '71': 'q',
//  '72': 'r',
//  '73': 's',
//  '74': 't',
//  '75': 'u',
//  '76': 'v',
//  '77': 'w',
//  '78': 'x',
//  '79': 'y',
//  '7A': 'z',
//  '7B': '{',
//  '7C': '|',
//  '7D': '}',
//  '7E': '~'}
