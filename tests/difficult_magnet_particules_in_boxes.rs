/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/56c04261c3fcf33f2d000534/rust
 * @Date: create in 2022/3/8 11:45 AM
 */

fn doubles(maxk: i32, maxn: i32) -> f64{
    0f64
}



#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;

    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-12;
        let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
    }

    fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
        assert_float_equals(doubles(maxk, maxn), exp);
    }

    #[test]
    fn basic_tests_doubles() {
        dotest(1, 10, 0.5580321939764581);
        dotest(10, 1000, 0.6921486500921933);
        dotest(10, 10000, 0.6930471674194457);
    }
}