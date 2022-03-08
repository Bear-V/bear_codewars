/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/5547cc7dcad755e480000004/train/rust
 * @Date: create in 2022/3/7 5:18 PM
 */

fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    // your code
    let sum:f64 = m as f64 * (m as f64 + 1f64) / 2f64;
    let mut vec = vec![];
    for i in 1..(m + 1) {
        let j = (sum as f64 - i as f64) / (i as f64 + 1f64);
        if j.fract() != 0.0 {
            continue;
        }
        let j = j as i32;
        if j != i && j < m + 1 {
            vec.push((i, j));
        }
    }
    vec
}

fn _remove_nb(m: i32) -> Vec<(i32, i32)> {
    let n: i64 = m as i64;
    let s: i64 = n * (n+1) / 2;
    ((s-n)/(n+1)..n)
        .filter(|i| (s-i) % (i+1) == 0)
        .map(|i| (i, (s-i) / (i+1)))
        .map(|(a, b)| (a as i32, b as i32))
        .collect()
}

fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
fn basics_remove_nb() {
    testing(6, vec![]);
    testing(26, vec![(15, 21), (21, 15)]);
    testing(100, vec![]);
    testing(101, vec![(55, 91), (91, 55)]);
    testing(102, vec![(70, 73), (73, 70)]);
}