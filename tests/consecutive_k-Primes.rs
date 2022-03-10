/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/573182c405d14db0da00064e/train/rust
 * @Date: create in 2022/3/9 10:51 AM
 */

fn consec_kprimes(k: i32, arr: Vec<i32>) -> i32 {
    let mut num = 0;
    if arr.len() <= 1 {
        return 0;
    }
    arr.iter().reduce(|x, next_x| {
        let k_p = k_prime(*x);
        let n_k_p = k_prime(*next_x);
        if k == k_p && k == n_k_p {
            num += 1;
        }
        next_x
    });
    num
}


fn k_prime(n: i32) -> i32 {
    let mut n = n;
    let mut count = 0;
    for i in 2..=n {
        while n % i == 0 {
            count += 1;
            n /= i;
        }
    }
    count
}

fn testing(k: i32, arr: Vec<i32>, exp: i32) -> () {
    assert_eq!(consec_kprimes(k, arr), exp)
}

#[test]
fn basics_consec_kprimes() {
    testing(2, vec![10081, 10071, 10077, 10065, 10060, 10070, 10086, 10083, 10078, 10076, 10089, 10085, 10063, 10074, 10068, 10073, 10072, 10075], 2);
    testing(6, vec![10064], 0);
    testing(1, vec![10054, 10039, 10053, 10051, 10047, 10043, 10037, 10034], 0);
    testing(3, vec![10158, 10182, 10184, 10172, 10179, 10168, 10156, 10165, 10155, 10161, 10178, 10170], 5);
    testing(2, vec![10110, 10102, 10097, 10113, 10123, 10109, 10118, 10119, 10117, 10115, 10101, 10121, 10122], 7);
}
