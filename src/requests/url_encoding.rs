use hashbrown::HashMap;
use serde_urlencoded::{self, de, ser};
use serde_derive::{self, Serialize, Deserialize};

#[macro_use]
use lazy_static;

// expects string to be lowercase
pub fn hex_to_char(input: &str) -> String {
    let input = input.to_uppercase();
    let mut input_clone = String::with_capacity(20);

    lazy_static!{
        static ref hex : HashMap<String, String> = _hex_to_char();
        static ref percent_encode: HashMap<String, String> = reserved_characters();
        static ref indexes :  Vec<usize> = (0..40).step_by(2).collect();
    }

    for i in indexes.iter(){
        let chars = input.get(*i..*i+2).unwrap();
        
        println!{"chars are {}", chars}

        match hex.get(chars) {
            // the two character combination matches something in hex
            Some(x) => { 
                // println!{"hex encoding found for character {}", x}
                //get escape character
                match percent_encode.get(x) {
                    // it has an escape character, push the escaped version
                    Some(escape_char) => {
                        // println!{"escape character found {}", escape_char}
                        input_clone.push_str(&escape_char)
                    },
                    // no escaped version, we can push the original
                    None => {
                        // println!{"no escape character found for {}",x};
                        input_clone.push_str(x);
                    }
                }
            },
            // The hex combination does not mean anything, push %<chars>
            None => {
                // println!{"no hex conversion for that character"}
                input_clone.push_str("%");
                input_clone.push_str(chars);
            }
        }
        dbg!{&input_clone};
    }

    return input_clone
}
pub fn _hex_to_char() -> HashMap<String, String> {
    let mut chars: Vec<&str> = "! \" # $ % & ' ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = > ? @ A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \\ ] ^ _ ` a b c d e f g h i j k l m n o p q r s t u v w x y z { | } ~".split_ascii_whitespace().collect();
    let letters = ["a", "b", "c", "d", "e", "f"];
    let mut hex: Vec<String> = Vec::new();
    chars.insert(0, " ");

    for i in (2..8){
        // let mut new : Vec<String> = Vec::new();
        
        for j in (i*10)..((i+1)*10){

            hex.push(j.to_string());
        }
        for j in 0..6 {
            let mut num = i.to_string();
            num.push_str(letters[j]);
            hex.push(num)
        }
    }
    let mut hm: HashMap<String, String> = HashMap::new();

    for i in 0..chars.len(){
        hm.insert(hex[i].clone(),chars[i].to_string().clone());
    }    
    
    return hm
}

fn reserved_characters() -> HashMap<String, String> {
    let mut keys: Vec<&str>= "! \" # $ % & ' ( ) * + , / : ; = ? @ [ ] < > - . ^ _ ` { | } ~".split_ascii_whitespace().collect();
    let mut values: Vec<&str> = "%20 %21 %22 %23 %24 %25 %26 %27 %28 %29 %2A \
    %2B %2C %2F %3A %3B %3D %3F %40 %5B %5D %3C %3E %2D %2E %5E %5F %60 %7B %7C %7D %7E".split_ascii_whitespace().collect();
    keys.insert(0, " ");
    println!{"{} {} ", keys.len(), values.len()}

    let mut hm: HashMap<String, String> = HashMap::new();
    for _ in 0..keys.len(){
        hm.insert(keys.remove(0).to_string(), values.remove(0).to_string());
    }
    return hm;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
	info_hash: String,
	peer_id: String,
	port: u32,
	uploaded: u32,
	downloaded: u32,
	numwant: u32,
	compact: u32
}

impl Url {
    pub fn new(info_hash: String, peer_id: String) -> Url {
        let url_hash     = hex_to_char(&info_hash).to_ascii_lowercase();
        let peer_id_hash = hex_to_char(&peer_id).to_ascii_lowercase();
        Url{info_hash: url_hash, peer_id: peer_id_hash, port: 9973, uploaded:0, downloaded:0,numwant:0,compact: 1}
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