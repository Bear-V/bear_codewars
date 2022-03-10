/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/561e9c843a2ef5a40c0000a4/train/rust
 * @Date: create in 2022/3/8 10:05 AM
 */
// 1) 设计 is_prime 用于判断单个函数是否质数 https://zhuanlan.zhihu.com/p/351701312?ivk_sa=1024320u
// 2) gap 函数中对 令 i 为 n~m 区间的数，如果 i 和 i +g 都是质数，再判断 (i,i+g)之间没有质数.

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let ug = g as u64;
    for i in m..=n {
        let next_i = i + ug;
        if is_prime(i) && is_prime(next_i) && (next_i < (n + 1)) {
            if (i + 1..(next_i)).all(|x| !is_prime(x)) {
                return Some((i, next_i));
            }
        }
    }
    None
}

fn is_prime(n:  u64) -> bool {
    n == 2 || n % 2 > 0 && (3..=(n as f64).sqrt() as u64).step_by(2).all(|i| n % i > 0)
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(gap(g, m, n), exp)
}

#[test]
fn basics_gap() {
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(6, 100, 110, None);
    testing(8, 300, 400, Some((359, 367)));
}
