
extern crate nyaa_tracker;
use nyaa_tracker::requests::url_encoding::hex_to_char;


fn base_test(to_convert: &str, result:&str) {
    let hm =hex_to_char(to_convert);
    assert_eq!(result, hm);
}

#[test]
fn hash_test() {
    let to_convert = "123456789abcdef123456789abcdef123456789a";
    let hm =hex_to_char(to_convert);
    assert_eq!("%124Vx%9a%bc%de%f1%23Eg%89%ab%cd%ef%124Vx%9a", hm)
}
#[test]
fn hash_test_2() {
    let to_convert = "6c1af1447f0cb430fa0a0fa1f94b7b32df926c4a";
    let result = "%6c%1a%f1D%7f%0c%b40%fa%0a%0f%a1%f9%4b%7b2%df%92%6c%4a";
    base_test(to_convert, result);
}
#[test]
fn hash_test_3() {
    let to_convert = "17b46b16818f1d753f6403ce48328976c13a4bfe";
    let result = "%17%b4%6b%16%81%8f%1du%3fd%03%ceH2%89v%c1%3a%4b%fe";
    base_test(to_convert, result);
}

#[test]
fn hash_test_4() {
    let to_convert = "d9261d6ea3e6a238cb30b898b917172f36c3a392";
    let result = "%d9%26%1d%6e%a3%e6%a28%cb0%b8%98%b9%17%17%2f6%c3%a3%92";
    base_test(to_convert, result);
}

#[test]
fn hash_test_5() {
    let to_convert = "3d96dfa67e7daac8e951a0e453820235becce439";
    let result = "%3d%96%df%a6%7e%7d%aa%c8%e9Q%a0%e4S%82%025%be%cc%e49";
    base_test(to_convert, result);
}

#[test]
fn hash_test_6() {
    let to_convert = "679bff2026b8a76ace66cffd08cbc27d89c0b864";
    let result = "g%9b%ff%20%26%b8%a7%6a%cef%cf%fd%08%cb%c2%7d%89%c0%b8d";
    base_test(to_convert, result);
}

#[test]
fn hash_test_7() {
    let to_convert = "c71783aa9ad4f5f2140f52705882fc33259b1b79";
    let result = "%c7%17%83%aa%9a%d4%f5%f2%14%0fRpX%82%fc3%25%9b%1by";
    base_test(to_convert, result);
}
#[test]
fn hash_test_8() {
    let to_convert = "3d211898763c858d8fb3456140b4a0b651468cb8";
    let result = "%3d%21%18%98v%3c%85%8d%8f%b3Ea%40%b4%a0%b6QF%8c%b8";
    base_test(to_convert, result);
}

#[test]
fn hash_test_9() {
    let to_convert = "038603179259c956ea2236cf55473564c92ec890";
    let result = "%03%86%03%17%92Y%c9V%ea%226%cfUG5d%c9%2e%c8%90";
    base_test(to_convert, result);
}

#[test]
fn hash_test_10() {
    let to_convert = "373a926c71a1ff1e53e107df49be6d24243e214c";
    let result = "7%3a%92%6cq%a1%ff%1eS%e1%07%dfI%be%6d%24%24%3e%21%4c";
    base_test(to_convert, result);
}

#[test]
fn hash_test_11() {
    let to_convert = "a51896742a9b5fd97bd7b519e78e4eb7f70666bc";
    let result = "%a5%18%96t%2a%9b%5f%d9%7b%d7%b5%19%e7%8e%4e%b7%f7%06f%bc";
    base_test(to_convert, result);
}

#[test]
fn hash_test_12() {
    let to_convert = "5ff149eba169f93d35e713413ae9f331149a63dc";
    let result = "%5f%f1I%eb%a1i%f9%3d5%e7%13A%3a%e9%f31%14%9ac%dc";
    base_test(to_convert, result);
}

#[test]
fn hash_test_13() {
    let to_convert = "bf39dd9b13b04589923ead4f434ee592988cebde";
    let result = "%bf9%dd%9b%13%b0E%89%92%3e%ad%4fC%4e%e5%92%98%8c%eb%de";
    base_test(to_convert, result);
}

#[test]
fn hash_test_14() {
    let to_convert = "456bc8be7b655fa0ae9d307f9aa9a890a2fd29db";
    let result = "E%6b%c8%be%7be%5f%a0%ae%9d0%7f%9a%a9%a8%90%a2%fd%29%db";
    base_test(to_convert, result);
}

#[test]
fn hash_test_15() {
    let to_convert = "cf7f23087183fe2525e4fdc73b33da8baa13bf64";
    let result = "%cf%7f%23%08q%83%fe%25%25%e4%fd%c7%3b3%da%8b%aa%13%bfd";
    base_test(to_convert, result);
}