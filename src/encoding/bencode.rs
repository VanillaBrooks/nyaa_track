#![feature(try_trait)]
use hashbrown;
use regex::Regex;
use std::cmp::Ordering;
use std::rc::Rc;

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

#[derive(Debug)]
pub enum DeBencode {
    Num(i32),
    Str(String),
    Dic(hashbrown::HashMap<String,DeBencode>),
    Lis(Vec<String>),
    Empty,
}
impl DeBencode {
    fn get_inner(&self) ->String {
        match self {
            DeBencode::Num(x) => {format!{"{:?}", x} },
            _ =>  {return "er".to_string()}
        }
    }
}

// impl try_trait for DeBencode {
//     type Ok = DeBencode;
//     type Error = Error;

//     fn into_result(self) -> Result<DeBencode, None> {
//         self.ok_or()
//     }
// }

#[derive(Debug)]
pub enum Error {
    Slice(ErrorLocation),
    Parse(ErrorLocation),
    Indexing(ErrorLocation),
    Lengths(ErrorLocation)
}
#[derive(Debug)]
pub enum FlatBencode{ 
    Num,
    Str,
    Dic,
    Lis,
    Empty
}

#[derive(Debug)]
pub enum ErrorLocation {
    General, 
    StrFind,
    StrParse,
    NumFind,
    NumParse,
    SingularExpr,
    Collections
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


pub fn general(input: &str) -> Result<Vec<DeBencode>, Error> {
    
    let collections_option = parse_for_lists_and_dicts(&input);

    match collections_option{
        None =>{
            // println!{"empty"}
            return Ok(vec![singular_expression(&input)?]);
        }
        Some(collections) =>{
            // println!{"handle collections"}
            let unwrapped = handle_colletions(&input, &collections)?;
            // return Ok(vec![unwrapped])
        }
    }


    return Ok(vec![DeBencode::Num(23)])
}


macro_rules! shitty_iter {
    ($x:expr, $input:expr, $funct:ident, $hm:ident) => {
        for location in $x{
            let returned_parse = $funct($input, &location.start, &location.end);

            match returned_parse{ 
                Ok(x) => $hm.insert(&location.start, Rc::new(x)),
                Err(x) => return Err(x)
            };
        }
    };
}

fn handle_colletions (input: &str, data:  &Collections_Data) -> Result<hashbrown::HashMap<String, Rc<DeBencode>>, Error> {
    let number_locations = find_all_numbers(&input)?;
    let string_locations = find_all_strings(&input)?;

    let mut hash : hashbrown::HashMap<&usize, Rc<DeBencode>> = hashbrown::HashMap::new();
    shitty_iter!(&number_locations, &input, parse_number, hash);
    shitty_iter!(&string_locations, &input, parse_string, hash);

    
    let mut indexes : Vec<_> = number_locations.iter().chain(string_locations.iter()).map(|x| x.start).collect::<Vec<_>>();
    indexes.sort();
    
    let mut total_dict: hashbrown::HashMap<String, Rc<DeBencode>>= hashbrown::HashMap::new();
    let mut i = 0;


    //
    //  Note: this does not yet handle nested collections. only 1 dictioanry will be searched
    //
    for _ in 0..indexes.len(){
        if i+1 >= indexes.len(){break}
        let c_index = &indexes[i];
        let c_index2 = &indexes[i+1];
        i += 2;

        let key = hash.get(c_index).unwrap().get_inner();
        let value = Rc::clone(&hash.get(c_index2).unwrap());

        total_dict.insert(key, value);
        
    }

    return Ok(total_dict);
}


fn singular_expression(input: &str) -> Result<DeBencode, Error> {
        let nums = find_all_numbers(&input)?;
        if nums.len() ==1{
            let s = &nums[0];
            return Ok(parse_number(&input, &s.start, &s.end)?)
        }
        else if nums.len() >= 1 {
            return Err(Error::Indexing(ErrorLocation::SingularExpr))
        }
        
        let strings = find_all_strings(&input)?;
        if strings.len() ==1 {
            let s = &strings[0];
            return Ok(parse_string(&input, &s.start, &s.end)?);
        }
        else if strings.len() >=1 {
            return Err(Error::Indexing(ErrorLocation::SingularExpr))
        }

        return Ok(DeBencode::Empty)
}
pub fn find_all_strings(input: &str) -> Result<Vec<Locations>, Error> {
    let str_start = Regex::new(r"\d:").unwrap();

    let starting_indicies : Vec<_> = str_start.find_iter(&input).map(|x| x.start()).collect();

    let mut output: Vec<Locations> = vec![];
    for i in starting_indicies {

        let starting_pos = input.get(i..i+1);

        let slice = match starting_pos {
            Some(x) => x,
            None => return Err(Error::Slice(ErrorLocation::StrFind)),
        };

        let num_start = match slice.parse::<usize>() {
            Ok(x) => x,
            Err(x) => return Err(Error::Parse(ErrorLocation::StrFind))
        };

        output.push(Locations::new(i+2, i+2+num_start));
    }

    return Ok(output)
}

fn parse_string(input: &str, start: &usize, end: &usize) -> Result<DeBencode, Error> {
    let slice = input.get(*start..*end);
    match slice {
        Some(x) => return Ok(DeBencode::Str(x.to_string())),
        None => return Err(Error::Slice(ErrorLocation::StrParse))
    }
}

#[derive(Debug)]
pub struct Locations {
    start: usize,
    end: usize
}
impl Locations{
    fn new(x: usize, y:usize)->Locations {
        Locations{start:x, end:y}
    }
}

fn find_all_numbers(input: &str) -> Result<Vec<Locations>, Error>{
    let num_start = Regex::new(r"i[\-\d]").unwrap();
    let num_end = Regex::new(r"\de").unwrap();

    let starting_indicies: Vec<_> = num_start.find_iter(&input).map(|x| x.start()+1).collect();
    let ending_indicies: Vec<_> = num_end.find_iter(&input).map(|x| x.end()-1).collect();

    if starting_indicies.len() != starting_indicies.len() {
        return Err(Error::Lengths(ErrorLocation::NumFind));
    }
    
    //TODO: preallocate this bad boy
    let mut locations: Vec<Locations> = Vec::new();
    for i in 0..starting_indicies.len(){
        locations.push(Locations::new(starting_indicies[i],ending_indicies[i]));
    }

    Ok(locations)
}

fn parse_number(input: &str, start: &usize, end: &usize) -> Result<DeBencode, Error> {
    
    let str_to_parse = input.get(*start..*end);
    let num = match str_to_parse {
        Some(x) => x,
        None => return Err(Error::Slice(ErrorLocation::NumParse))
    };
    match num.parse::<i32>(){
        Ok(x) => Ok(DeBencode::Num(x)),
        Err(x) => Err(Error::Parse(ErrorLocation::NumParse))
    }

}

#[derive(Debug)]
struct Collections_Data {
    pub dict_locations: Vec<Locations>,
    pub list_locations: Vec<Locations>
}

impl Collections_Data {
    fn new(dicts: Vec<Locations>, lists: Vec<Locations>) -> Collections_Data{
        return Collections_Data{dict_locations: dicts, list_locations: lists}
    }
    fn is_empty(&self) -> bool {
        return (self.dict_locations.len()>0) || (self.list_locations.len() >0)
    }
}

fn parse_for_lists_and_dicts(input: &str) -> Option<Collections_Data> {

	let dict_start = Regex::new(r"d[l\di]").unwrap();
	let lis_start = Regex::new(r"l[idl\d]").unwrap();
	let collection_end = Regex::new(r"ee|\d:[[:alpha:]]+e^[i\dd]").unwrap();

    let dict_starts: Vec<_>= dict_start.find_iter(&input).map(|x| (x.start()+1)).collect();
    let collection_ends: Vec<_> = collection_end.find_iter(&input).map(|x| (x.end()-1)).collect();
    let list_starts: Vec<_> = lis_start.find_iter(&input).map(|x| (x.start()+1)).collect();

    if dict_starts.len() + list_starts.len() != collection_ends.len() {
        panic!{"number of dictionary starts ({:?}) and list starts ({:?}) did not equal dictionary ends ({:?}) for phrase {}",dict_starts, list_starts, collection_ends, input}
    }

    if (dict_starts.len()==0) && (list_starts.len()==0){
        return None
    }

    let decision = dict_starts.len().cmp(&list_starts.len());
    let (primary_iter, secondary_iter, order) = match decision{
        Ordering::Less => (&list_starts, &dict_starts, FlatBencode::Lis),
        _ =>              (&dict_starts, &list_starts, FlatBencode::Dic)
    };

    
    let mut final_dict: Vec<Locations> = Vec::new();
    let mut final_list: Vec<Locations> = Vec::new();

    let mut dict_i = 0;
    let mut list_i= 0;

    for _ in 0..collection_ends.len(){

        let decision_prim = dict_i.cmp(&primary_iter.len());
        let prim = match decision_prim{
            (Ordering::Less) => primary_iter[dict_i],
            _ => 0
        };

        let decision_sec = list_i.cmp(&secondary_iter.len());
        let sec = match decision_sec {
            Ordering::Less => secondary_iter[list_i],
            _ => 0,
        };

        let ender = collection_ends[dict_i+list_i];

        if (ender > sec) && (sec> prim) {
            final_list.push(Locations::new(sec,ender));
            list_i +=1;
        }
        else if (ender > prim) && (prim > sec) {
            final_dict.push(Locations::new(prim,ender));
            dict_i +=1;
        }
        else{panic!{"conditions not met for bencode :::{}:::\nender {}\nsec {}\nprim {}\n", input, ender, sec, prim}}

    }

    return Some(Collections_Data::new(final_dict, final_list));
}

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