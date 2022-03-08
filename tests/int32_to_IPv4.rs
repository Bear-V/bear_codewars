/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/52e88b39ffb6ac53a400022e/train/rust
 * @Date: create in 2022/3/8 3:02 PM
 */
use std::net::Ipv4Addr;
fn int32_to_ip(int: u32) -> String {
    Ipv4Addr::from(int).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}
