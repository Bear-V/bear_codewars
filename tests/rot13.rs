use std::char;

/**
 * @Author: ZZX
 * @Description: rot13
 * @Date: create in 2022/3/3 6:46 PM
 */

///ROT13 is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. ROT13 is an example of the Caesar cipher.
//
// Create a function that takes a string and returns the string ciphered with Rot13. If there are numbers or special characters included in the string, they should be returned as they are. Only letters from the latin/english alphabet should be shifted, like in the original Rot13 "implementation".


fn rot13(message: &str) -> String {
    let v = vec![("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm")];
    // your code here
    let a = message.as_bytes();
    for i in a {
        println!("{}", char::from(*i));
    }

    String::new()
}

#[test]
fn one() {
    rot13("abcdefg");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_fixed() {
//         assert_eq!(rot13("test"), "grfg");
//         assert_eq!(rot13("Test"), "Grfg");
//     }
// }