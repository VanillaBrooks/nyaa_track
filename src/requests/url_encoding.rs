// def hex_to_char_dict():
// 	letters = "ABCDEF"
// 	hex = []
// 	for k in range(2,8):
// 		new = [str(i) for i in range(k*10,(k+1)*10)] + [str(k) + letters[j] for j in range(6)]
// 		hex += new
// 	hex.pop(-1)
// 	chars = ' ! " # $ % & \' ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = > ? @ A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \\ ] ^ _ ` a b c d e f g h i j k l m n o p q r s t u v w x y z { | } ~'.split(' ')
// 	return {hex[i]: chars[i] for i in range(len(hex))}

use hashbrown::HashMap;

use lazy_static;


// lazy_static!{
// 		static ref hm: HashMap<String, String> = hex_to_char();
// 	}
pub fn hex_to_char() -> HashMap<String, String> {
    let mut chars: Vec<&str> = "  ! \" # $ % & ' ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = > ? @ A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \\ ] ^ _ ` a b c d e f g h i j k l m n o p q r s t u v w x y z { | } ~".split_ascii_whitespace().collect();
    let letters = ["A", "B", "C", "D", "E", "F"];
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


// {'20': '',
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