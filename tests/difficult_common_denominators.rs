/**
 * @Author: BearV
 * @Description: 最小公倍数 0 0 https://www.codewars.com/kata/54d7660d2daf68c619000d95
 * @Date: create in 2022/3/7 11:49 AM
 */

fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // your code
    let mut arr = vec![];
    for i in 0..l.len() {
        arr.push(l[i].1)
    }
    let d = more_lcm(arr);
    println!("den: {}", d);
    let d = l.iter().fold(1, |acc, &(num, den)| lcm(acc, den / gcd(num, den)));
    println!("den: {}", d);

    l.iter().map(|(num, den)| (num * d / den, d)).collect::<Vec<(i64, i64)>>()
}

// 两数的最大公约数
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    let c = a % b;
    gcd(b, c)
}

// 两数的最小公倍数
fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

// 多个数的最小公倍数 错误
fn more_lcm(mut arr: Vec<i64>) -> i64 {
    for i in 0..arr.len() - 1 {
        arr[i + 1] = lcm(arr[i], arr[i + 1])
    }
    arr[arr.len() - 1]
}


// //最大公约数
// fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }
//
// //最小公倍数
// fn lcm(a: i64, b: i64) -> i64 { a / gcd(a, b) * b }
//
// fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
//     // 多个数的最小公倍数
//     let d = l.iter().fold(1, |acc, &(num, den)| lcm(acc, den / gcd(num, den)));
//     l.iter().map(|&(num, den)| (num * d / den, d)).collect()
// }


fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(convert_fracts(l), exp)
}

#[test]
fn basics_convert_fracts() {
    testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
}