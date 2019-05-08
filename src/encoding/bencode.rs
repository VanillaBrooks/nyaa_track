use hashbrown;
use regex::Regex;
use std::cmp::Ordering;

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
macro_rules! ben_encode {
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

struct Decoding_Positions {
    pub ty: FlatBencode,
    pub start: u32,
    pub end: u32,
    pub contained_str: String
}

impl Decoding_Positions{
    fn new(ty: FlatBencode, start:u32, end:u32, contained_str: String)->Decoding_Positions{
        return Decoding_Positions{ty:ty, start:start, end:end, contained_str: contained_str}
    }
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



pub fn general(input: &str) -> Result<DeBencode, &str> {

	let dict_start = Regex::new(r"d[l\di]").unwrap();

	let lis_start = Regex::new(r"l[idl\d]").unwrap();
	let collection_end = Regex::new(r"ee|\d:[[:alpha:]]+e^[i\dd]").unwrap();

    // let dict_start = dict_start.find(&input);
    // let dict_end = dict_end.find(&input);
    
    // match 
    let dict_starts: Vec<_>= dict_start.find_iter(&input).map(|x| (x.start()+1) as i32).collect();
    let collection_ends: Vec<_> = collection_end.find_iter(&input).map(|x| (x.end()-1) as i32).collect();
    let list_starts: Vec<_> = lis_start.find_iter(&input).map(|x| (x.start()+1) as i32).collect();

    if dict_starts.len() + list_starts.len() != collection_ends.len() {
        panic!{"number of dictionary starts ({:?}) did not equal dictionary ends ({:?}) for phrase {}",dict_starts, collection_ends, input}
    }

    println!{"all dicts {:?}", dict_starts}
    println!{"all lists {:?}" ,list_starts}
    println!{"all of collections: {:?}", collection_ends}

    // this may be error prone
    let primary_iter = &list_starts;
    let secondary_iter = &dict_starts;
    let order = FlatBencode::Lis;
    if dict_starts.len() > list_starts.len(){
        println!{"primary is dict"}
        let primary_iter = &dict_starts;
        let secondary_iter = &list_starts;
        let order = FlatBencode::Dic;
    }
    else{
        println!{"primary is lists"}
    }
    
    // TODO: Preallocate these 
    let mut final_prim: Vec<(i32, i32)> = Vec::new();
    let mut final_sec: Vec<(i32, i32)> = Vec::new();

    let mut prim_i = 0;
    let mut sec_i = 0;

    for i in 0..collection_ends.len(){
        println!{"\n\n"}
        // println!{"here"}

        let decision_prim = prim_i.cmp(&primary_iter.len());
        let prim = match decision_prim{
            (Ordering::Less) => primary_iter[prim_i],
            _ => -1i32
        };

        let decision_sec = sec_i.cmp(&secondary_iter.len());
        let sec = match decision_sec {
            (Ordering::Less) => secondary_iter[sec_i],
            _ => -1i32,
        };
        println!{"made it this far"}
        let ender = collection_ends[prim_i+sec_i];
        println!{"over ehre"}

        if sec_i < secondary_iter.len(){
            println!{"assigning a new value to sec"}
            let sec =  secondary_iter[sec_i];
        }
        else{
            println!{"we are keeping sec as 0 {} {}", sec_i, secondary_iter.len()}
        }

        println!{"primary is {} secondary is {} ender is {}", prim, sec, ender}

        if (ender > sec)&& (sec> prim) {
            final_sec.push((sec,ender));
            sec_i +=1
        }
        else if (ender > prim) && (prim > sec) {
            final_prim.push((prim,ender));
            prim_i +=1;
        }
        panic!{"conditions not met for bencode :::{}:::", input}

        // // if the distacne between the end and the secondary is smaller 
        // if sec < 0{
        //     final_prim.push((prim,ender));
        //     prim_i +=1;
        // }
        // else if prim < 0{
        //     final_sec.push((sec, ender));
        //     sec_i+=1
        // }
        // else if (ender-sec) <0 {
        //     println!{"first condition"}
        //     final_prim.push((prim,ender));
        //     prim_i +=1;
        // }
        // else if (ender-prim) <0 {
        //     println!{"second condition {}", ender-prim}
        //     final_sec.push((sec, ender));
        //     sec_i+=1
        // }
        // else if (ender-prim) < (ender-sec){
        //     println!{"3rd condition"}
        //     final_prim.push((prim, ender));
        //     prim_i +=1;
        // }
        // else if (ender-sec) < (ender-prim){
        //     println!{"4th condition"}
        //     final_sec.push((prim, ender));
        //     sec_i+=1;
        // }
        // else {panic!{"conditions not met for bencode :::{}:::", input}}

    println!{"END OF LOOP:"}
    println!{"primary {:?}", final_prim}
    println!{"secondary {:?}", final_sec}
    }
    

    // let num_start = Regex::new(r"i[\-\d]").unwrap();
    // let num_end = Regex::new(r"\de").unwrap();

    // let str_start = Regex::new(r"\d:").unwrap();

    // let lis_start = Regex::new(r"l[idl\d]").unwrap();
	// let lis_end = Regex::new(r"ee|\d:[[:alpha:]]+e^[i\dd]").unwrap();

    // let dict_starts = 

    


    
    return Ok(DeBencode::Num(23))

}

// D<contents>E
pub fn dic(input: &str) -> Result<DeBencode, &str> {



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

#[macro_export]
macro_rules! ben_decode {
    ($input:expr) => {
        let vec : Vec<DeBencode> = Vec::new();
        let positions = general(input);

        for i in general($x){
            match i.ty{
                FlatBencode::Num =>{
                    ben_decode!(num: x, input.get(i.start..i.end))
                }
                FlatBencode::Str =>{}
                FlatBencode::Dic=>{}
                FlatBencode::Lis=> {}
                FlatBencode::Empty =>{}

                
            }
        } 
    };
    (num: $start:expr, $end:expr) =>{}

}







// TEST CASES ENCODING
	// dbg!(bencode!{"one_word"});
	// dbg!(bencode!{["sequence", "of", "values"]});
	// dbg!(bencode!{{"sample":"dictionary","multiple":"values"}});
	// dbg!(bencode!{raw: "raw_one_word"});
	// dbg!(bencode!{raw: 234i32});
	// println!{"{:?}", 23i32.bencode()}