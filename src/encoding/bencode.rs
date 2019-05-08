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


pub fn general(input: &str) -> Result<Vec<DeBencode>, &str> {
    
    let collections = parse_for_lists_and_dicts(&input);

    if collections.is_empty(){
        let nums = find_all_numbers(&input)?;
        if nums.len() ==0{
            let s = &nums[0];
            return Ok(vec![parse_number(&input, s.start, s.end)?])
        }
        else if nums.len() >= 1 {
            return Err("We found more than one number after no dicts and lists were found. this should not happen")
        }
        // else {
        //     let strings + find_all_strings(&input)?;
        // }
        
    }
    dbg!{collections};


    // let dict_starts = 

    return Ok(vec![DeBencode::Num(23)])

}
pub fn find_all_strings(input: &str) -> Result<Vec<Locations>, &str> {
    let str_start = Regex::new(r"\d:").unwrap();

    let starting_indicies : Vec<_> = str_start.find_iter(&input).map(|x| x.start()).collect();

    let mut output: Vec<Locations> = vec![];
    for i in starting_indicies {

        let starting_pos = input.get(i..i+1);

        let slice = match starting_pos {
            Some(x) => x,
            None => return Err("could not correctly slice the string in find_all_strings"),
        };

        let num_start = match slice.parse::<usize>() {
            Ok(x) => x,
            Err(x) => return Err("could not parse the byte count from find_all_strings")
        };

        output.push(Locations::new(i+2, i+2+num_start));
    }

    return Ok(output)
}

fn parse_string(input: &str, start: usize, end: usize) -> Result<DeBencode, &str> {
    let slice = input.get(start..end);
    match slice {
        Some(x) => return Ok(DeBencode::Str(x.to_string())),
        None => return Err("could not slice the string correctly in parse_string")
    }
}

pub struct Locations {
    start: usize,
    end: usize
}
impl Locations{
    fn new(x: usize, y:usize)->Locations {
        Locations{start:x, end:y}
    }
}
fn find_all_numbers(input: &str) -> Result<Vec<Locations>, &str>{
    let num_start = Regex::new(r"i[\-\d]").unwrap();
    let num_end = Regex::new(r"\de").unwrap();

    let starting_indicies: Vec<_> = num_start.find_iter(&input).map(|x| x.start()+1).collect();
    let ending_indicies: Vec<_> = num_end.find_iter(&input).map(|x| x.end()-1).collect();

    if starting_indicies.len() != starting_indicies.len() {
        return Err("the starting indicies and the endinging indicies did not have the same length while number parsing");
    }
    
    //TODO: preallocate this bad boy
    let mut locations: Vec<Locations> = Vec::new();
    for i in 0..starting_indicies.len(){
        locations.push(Locations::new(starting_indicies[i],ending_indicies[i]));
    }

    Ok(locations)
}
fn parse_number(input: &str, start: usize, end:usize) -> Result<DeBencode, &str> {
    
    let str_to_parse = input.get(start..end);
    let num = match str_to_parse {
        Some(x) => x,
        None => return Err("encountered an error in parse_number while slicing the string")
    };
    match num.parse::<i32>(){
        Ok(x) => Ok(DeBencode::Num(x)),
        Err(x) => Err("was not able to correctly parse the number in parse_number")
    }

}
#[derive(Debug)]
struct Collections_Data {
    dict_locations: Vec<(i32, i32)>,
    list_locations: Vec<(i32, i32)>
}
impl Collections_Data {
    fn new(dicts: Vec<(i32, i32)>, lists: Vec<(i32, i32)>) -> Collections_Data{
        return Collections_Data{dict_locations: dicts, list_locations: lists}
    }
    fn is_empty(&self) -> bool {
        return (self.dict_locations.len()>0) || (self.list_locations.len() >0)
    }
}

fn parse_for_lists_and_dicts(input: &str) -> Collections_Data {
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

    let decision = dict_starts.len().cmp(&list_starts.len());
    let (primary_iter, secondary_iter, order) = match decision{
        Ordering::Less => (&list_starts, &dict_starts, FlatBencode::Lis),
        _ =>              (&dict_starts, &list_starts, FlatBencode::Dic)
    };

    
    // TODO: Preallocate these 
    let mut final_prim: Vec<(i32, i32)> = Vec::new();
    let mut final_sec: Vec<(i32, i32)> = Vec::new();

    let mut prim_i = 0;
    let mut sec_i = 0;

    for i in 0..collection_ends.len(){

        let decision_prim = prim_i.cmp(&primary_iter.len());
        let prim = match decision_prim{
            (Ordering::Less) => primary_iter[prim_i],
            _ => -1i32
        };

        let decision_sec = sec_i.cmp(&secondary_iter.len());
        let sec = match decision_sec {
            Ordering::Less => secondary_iter[sec_i],
            _ => -1i32,
        };
        let ender = collection_ends[prim_i+sec_i];

        if sec_i < secondary_iter.len(){
            let sec =  secondary_iter[sec_i];
        }


        if (ender > sec) && (sec> prim) {
            final_sec.push((sec,ender));
            sec_i +=1
        }
        else if (ender > prim) && (prim > sec) {
            final_prim.push((prim,ender));
            prim_i +=1;
        }
        else{panic!{"conditions not met for bencode :::{}:::\nender {}\nsec {}\nprim {}\n", input, ender, sec, prim}}

    }
    match order {
        FlatBencode::Lis => return Collections_Data::new(final_sec, final_prim),
        _ => return Collections_Data::new(final_prim, final_sec),
    }
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