/**
 * @Author: ZZX
 * @Description: 乌龟赛跑 https://www.codewars.com/kata/55e2adece53b4cdcb900006c/train/rust
 * @Date: create in 2022/3/4 1:14 PM
 */
/// Two tortoises named A and B must run a race. A starts with an average speed of 720 feet per hour. Young B knows she runs faster than A, and furthermore has not finished her cabbage.
//
// When she starts, at last, she can see that A has a 70 feet lead but B's speed is 850 feet per hour. How long will it take B to catch A?
//
// More generally: given two speeds v1 (A's speed, integer > 0) and v2 (B's speed, integer > 0) and a lead g (integer > 0) how long will it take B to catch A?
//
// The result will be an array [hour, min, sec] which is the time needed in hours, minutes and seconds (round down to the nearest second) or a string in some languages.
//
// If v1 >= v2 then return nil, nothing, null, None or {-1, -1, -1} for C++, C, Go, Nim, Pascal, COBOL, [-1, -1, -1] for Perl,[] for Kotlin or "-1 -1 -1".
//
// Examples:
fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    let second = 3600 * g / (v2 - v1);
    if second < 0 {
        return None;
    }
    let mut v = vec![0; 3];
    let h = second / 3600;
    let s = second - 3600 * h;
    let mm = s / 60;
    let s = s - 60 * mm;
    v[0] = h;
    v[1] = mm;
    v[2] = s;
    Some(v)
}

fn _race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        None
    } else {
        let d = (3600 * g) / (v2 - v1);
        Some(vec![d / 3600, d / 60 % 60, d % 60])
    }
}

#[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 81, 550), None);
}