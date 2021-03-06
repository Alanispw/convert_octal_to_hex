fn main() {
}

pub fn octal_to_dec(mut _octal: i32) -> i32 {
    let oct: i32 = 8;
    let mut _decimal: i32 = 0; 
    let mut rem;
    let mut place = 0; 

    //Convert octal to decimal 
    while _octal != 0 {
        rem = _octal % 10;
        _decimal = _decimal + rem * oct.pow(place);
        _octal = _octal / 10; 
        place += 1; 
    }
    return _decimal;
}

pub fn dec_to_hex(mut _decimal: i32) -> String {
    let mut j: u32 = 1; 
    let dec: i32 = 16;
    let mut i = 1;
    let mut dec_search: i32 = _decimal;
    let mut _hexadecimal = String::new();
    
    //Convert decimal to hexadecimal
    while 1 > 0 {
        if _decimal >= dec.pow(i) {
            j = j + 1; 
        }
        else if _decimal < dec.pow(i) {
            break;
        }
        i += 1;
    }

    for n in 0..j {
        _hexadecimal.push_str(&search_hex(dec_search / dec.pow(j - 1 - n )));
        dec_search = dec_search % dec.pow(j - 1 - n);
    }
    return _hexadecimal;
}

//Compare numbers 
fn search_hex(num: i32) -> String {
    match num {
        num if num == 0 => "0".to_string(),
        num if num == 1 => "1".to_string(),
        num if num == 2 => "2".to_string(),
        num if num == 3 => "3".to_string(),
        num if num == 4 => "4".to_string(),
        num if num == 5 => "5".to_string(),
        num if num == 6 => "6".to_string(),
        num if num == 7 => "7".to_string(),
        num if num == 8 => "8".to_string(),
        num if num == 9 => "9".to_string(),
        num if num == 10 => "A".to_string(),
        num if num == 11 => "B".to_string(),
        num if num == 12 => "C".to_string(),
        num if num == 13 => "D".to_string(),
        num if num == 14 => "E".to_string(),
        num if num == 15 => "F".to_string(),
        _ => "".to_string(),
    }
}

#[cfg(test)] 

mod converter_tests {

    use super::*;

    #[test] 
    fn test1_convert_octal1_to_dec1 () {
        let octal = 1;
        let decimal = octal_to_dec(octal);
        assert_eq!(decimal, 1);
    }
    #[test] 
    fn test2_convert_octal10_to_dec8  () {
        let octal = 10;
        let decimal = octal_to_dec(octal);
        assert_eq!(decimal, 8);
    }
    #[test] 
    fn test3_convert_octal345_to_dec229 () {
        let octal = 345;
        let decimal = octal_to_dec(octal);
        assert_eq!(decimal, 229);
    }
    #[test] 
    fn test4_convert_octal345_to_hex_e5 () {
        let octal = 345;
        let decimal = octal_to_dec(octal);
        let hexadecimal = dec_to_hex(decimal);
        assert_eq!(hexadecimal, "E5");
    }
    #[test] 
    fn test5_convert_octal372_to_hex_fa () {
        let octal = 372;
        let decimal = octal_to_dec(octal);
        let hexadecimal = dec_to_hex(decimal);
        assert_eq!(hexadecimal, "FA");
    }
    #[test] 
    fn test6_convert_octal1266_to_hex_2b6 () {
        let octal = 1266;
        let decimal = octal_to_dec(octal);
        let hexadecimal = dec_to_hex(decimal);
        assert_eq!(hexadecimal, "2B6");
    }
}
