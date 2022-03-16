/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/562e274ceca15ca6e70000d3/train/rust
 * @Date: create in 2022/3/11 2:53 PM
 */

// 抄 https://gist.github.com/barakplasma/08d106f5af8fd7afa94651760cc92960
struct Point {
    x: f64,
    y: f64
}

fn f(x: f64) -> f64 {
    x*x
}

const START: f64 = 0.;
const END: f64 = 1.;

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y
        }
    }
    pub fn distance(&self, pos: &Point) -> f64 {
        (&self.x - pos.x).hypot(&self.y - pos.y)
    }
}

fn len_curve(n: i32) -> f64 {
    let h = END / n as f64;
    (0..=n).fold((Point::new(START, f(START)), 0.), |acc, step| {
        let (last_point, sum_distances) = acc;
        let x = step as f64 * h;
        let current_point = Point::new(x, f(x));
        let new_sum_length = sum_distances + last_point.distance(&current_point);
        (current_point, new_sum_length)
    }).1
}


fn _len_curve(n: i32) -> f64 {
    let h = 1./n as f64;
    let f = |x: f64| x.powf(2.);
    (0..n).map(|i| h.hypot(f((i+1) as f64 * h) - f(i as f64 * h))).sum()
}

use float_eq::float_eq;

fn assert_float_equals(actual: f64, expected: f64) {
    let merr = 1.0e-6;
    let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
    assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
}

fn dotest(n: i32, exp: f64) -> () {
    assert_float_equals(len_curve(n), exp);
}

#[test]
fn basic_tests_len_curve() {
    dotest(1, 1.414213562);
    dotest(10, 1.478197397);
    dotest(40, 1.478896272);
    dotest(200, 1.478940994);
    dotest(1200, 1.478942805);
}