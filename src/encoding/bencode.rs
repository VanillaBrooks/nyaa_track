use hashbrown;
use regex::Regex;
pub mod percent {
    pub fn encode(){

    }
}

// TODO: preallocation of strings based on input size

#[derive(Debug)]
pub enum Bencoding {
    Str(String),
    Num(String),
    Lis(String),
    Dic(String),
}

impl From<i32> for Bencoding{
    fn from(input_num:i32) -> Self {
        // Bencoding::number(format!("i{:?}e", input_num))
        Bencoding::Num(input_num.bencode())

    }
}
impl From<&str> for Bencoding{
    fn from(input_str: &str) -> Self {
        Bencoding::Str(input_str.bencode())
        // Bencoding::string(format!("{}:{}",input_str.len(), input_str))
    }
}

pub trait Bencode {
    fn bencode(&self) -> String;
}

impl Bencode for i32{
    fn bencode(&self) -> String {
        format!("i{:?}e", self)
    }
}

impl Bencode for &str {
    fn bencode(&self) -> String {
        format!("{}:{}",self.len(), self)
    }
}

#[macro_export]
macro_rules! bencode {
    ([$($element:tt),*]) =>{
        $crate::encoding::bencode::Bencoding::Lis(
            vec![
                "l".to_string(), 
                vec![$(bencode!(raw: $element)),*].concat(), 
                "e".to_string()
            ].concat()
        )

    };
    ( {$($left:tt : $right:tt),*}  ) => {
        $crate::encoding::bencode::Bencoding::Dic(
            vec![ "d".to_string(),
                vec![
                        $(bencode!(raw: $left),
                            bencode!(raw: $right)),*
                    ].concat(),
                "e".to_string()
            ].concat()
            )
    };
    (raw: $x:tt) => {
        $crate::encoding::bencode::Bencode::bencode(&$x)
    };
    ($other:tt) => {
        $crate::encoding::bencode::Bencoding::from($other)
    };
}

pub enum DeBencode {
    Num(i32),
    Str(String),
    Dic(hashbrown::HashMap<String,String>),
    Lis(Vec<String>)
}
#[derive(Debug)]
pub enum FlatBencode{ 
    Num,
    Str,
    Dic,
    Lis,
    Empty
}

fn check_letter(input: &str, character: &str) -> bool {
    match input.get(0..1) {
        Some(x) => {
            if x == character{
                return true
            }
            return false
        }
        None => return false
    }

}

pub fn get_next_stop(input_str: &str) -> FlatBencode {
    let re = Regex::new(r"[dli]-*\d|\d[:]").unwrap();
    let k = re.find(input_str).unwrap();
    let start = k.start();
    dbg!{k};
    match input_str.get(start..start+1){
        Some("d") => FlatBencode::Dic,
        Some("l") => FlatBencode::Lis,
        Some("i") => FlatBencode::Num,
        Some(_) => FlatBencode::Str,
        None => FlatBencode::Empty
    }
}

// D<contents>E
pub fn dic(input: &str) -> Result<DeBencode, &str> {
    if !check_letter(&input, "d"){return Err("parsing dictionary without correct letter starts")}

    let content_to_check = input.get(1..input.len()).unwrap();
    
    let end_dict = Regex::new(r"[e]&&\d{1}|[i]&&\d+|-{1}").unwrap();

    let num = "sampleei34e";
    let stri  = "";
    let list = "";
    let dic = "";

    return Ok(DeBencode::Num(23))
    

}
// // L<contents>E
// fn lis(input: &str) -> Result<DeBencode, String> {
//     return Result<DeBencode::Num(23)>
// }
// // I<num>E
// fn num(input: &str) ->Result<DeBencode, String> {
//     return Result<DeBencode::Num(23)>
// }
// //bytecount : string
// fn stri(input: &str) ->Result<DeBencode, String>{
//     return Result<DeBencode::Num(23)>
// }









// TEST CASES ENCODING
	// dbg!(bencode!{"one_word"});
	// dbg!(bencode!{["sequence", "of", "values"]});
	// dbg!(bencode!{{"sample":"dictionary","multiple":"values"}});
	// dbg!(bencode!{raw: "raw_one_word"});
	// dbg!(bencode!{raw: 234i32});
	// println!{"{:?}", 23i32.bencode()}