/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/554f76dca89983cc400000bb/train/rust
 * @Date: create in 2022/3/14 11:43 AM
 */


fn solequa(n: u64) -> Vec<(u64, u64)> {
    vec![(0, 0)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(solequa(n), exp)
    }

    #[test]
    fn basics_solequa() {
        testing(5, vec![(3, 1)]);
        testing(20, vec![(6, 2)]);
        testing(9001, vec![(4501, 2250)]);
        testing(9004, vec![(2252, 1125)]);
    }
}