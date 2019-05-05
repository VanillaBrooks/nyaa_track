extern crate hashbrown;
pub mod percent {
    pub fn encode(){

    }
}

// TODO: preallocation of strings based on input size
#[macro_use]

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

