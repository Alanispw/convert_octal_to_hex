fn main() {
}

pub fn octal_to_dec(mut _octal: i32) -> i32 {
    let oct: i32 = 8;
    let mut _decimal: i32 = 0; 
    let mut rem;
    let mut place = 0; 

    while _octal != 0 {
        rem = _octal % 10;
        _decimal = _decimal + rem * oct.pow(place);
        _octal = _octal / 10; 
        place += 1; 
    }
    return _decimal;
}

#[cfg(test)] 

mod converter_tests {

    use super::*;

    #[test] 
    fn test_convert_octal1_to_dec1 () {
        let octal = 1;
        let decimal = octal_to_dec(octal);
        assert_eq!(decimal, 1);
    }
    #[test] 
    fn test_convert_octal10_to_dec8  () {
        let octal = 10;
        let decimal = octal_to_dec(octal);
        assert_eq!(decimal, 8);
    }
    #[test] 
    fn test_convert_octal345_to_dec229 () {
        let octal = 345;
        let decimal = octal_to_dec(octal);
        assert_eq!(decimal, 229);
    }
}
