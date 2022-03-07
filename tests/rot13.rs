use std::char;

/**
 * @Author: ZZX
 * @Description: rot13
 * @Date: create in 2022/3/3 6:46 PM
 */

///ROT13 is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. ROT13 is an example of the Caesar cipher.
//
// Create a function that takes a string and returns the string ciphered with Rot13. If there are numbers or special characters included in the string, they should be returned as they are. Only letters from the latin/english alphabet should be shifted, like in the original Rot13 "implementation".
fn rotate_by_basis(ch: char, basis: char) -> char {
    (((ch as u8 - basis as u8 + 13) % 26) + basis as u8) as char
}

fn rot13(message: &str) -> String {
    // your code here
    message.chars().map(|ch| {
        if ch.is_ascii() {
            if ch.is_uppercase() {
                return rotate_by_basis(ch, 'A');
            } else if ch.is_lowercase() {
                return rotate_by_basis(ch, 'a');
            }
        }

        return ch;
    }).collect::<String>()
}

fn _rot13(message: &str) -> String {
    message.chars().map(|c| {
        match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - 13) as char,
            _ => c,
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}