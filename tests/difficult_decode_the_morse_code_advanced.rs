/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/54b72c16cd7f5154e9000457/train/rust
 * @Date: create in 2022/3/8 5:48 PM
 */
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

pub fn decode_bits(encoded: &str) -> String {
    let mut uints = 0;
    for i in encoded.chars() {
        if i == '0' {
            uints += 1;
        } else {
            break;
        };
    };

    let count = 1;
    for  in  {
        
    }
    
    
    let new_bit =&encoded[uints..];
    println!("uints :{},new_bit: {}",uints, new_bit);
    "".to_string()
}

pub fn decode_morse(encoded: &str) -> String {
    unimplemented!();
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[test]
fn examples() {
    // decode_bits("001100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011");
    assert_eq!(decode_morse(&decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}