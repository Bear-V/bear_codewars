/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/rust
 * @Date: create in 2022/3/11 2:20 PM
 */

fn _bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h <= 0. || window >= h || bounce <= 0. || bounce >= 1. {
        return -1;
    }

    let mut count: i32 = 1;
    let mut bounce_h = h * bounce;
    println!("bounce_h {}", bounce_h);
    while bounce_h > window {
        count += 2;
        bounce_h *= bounce;
        println!("bounce_h {}", bounce_h);
    }

    if count > 0 {
        count
    } else {
        -1
    }
}

fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
        -1
    } else {
        (window / h).log(bounce).ceil() as i32 * 2 - 1
    }
}


fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

#[test]
fn tests_bouncing_ball() {
    testequal(3.0, 0.66, 1.5, 3);
    testequal(30.0, 0.66, 1.5, 15);
    testequal(40.0, 0.4, 10.0, 3);
    testequal(10.0, 0.6, 10.0, -1);
}