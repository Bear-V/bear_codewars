/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/5511b2f550906349a70004e1
 * @Date: create in 2022/3/7 3:40 PM
 */

///0 结尾的数的任意次方总是 0
// 1 结尾的数的任意次方总是 1
// 2 结尾的数的任意次方总是 2, 4, 8, 6….
// 3 结尾的数的任意次方总是 3, 9, 7, 1….
// 4 结尾的数的任意次方总是 4, 6….
// 5 结尾的数的任意次方总是 5
// 6 结尾的数的任意次方总是 6
// 7 结尾的数的任意次方总是 7, 9, 3, 1….
// 8 结尾的数的任意次方总是 8, 4, 2, 6….
// 9 结尾的数的任意次方总是 9, 1….
fn _last_digit(str1: &str, str2: &str) -> i32 {
    let mut sum = 1;
    let mut a = str1.parse::<i64>().unwrap();
    let mut b = str2.parse::<i64>().unwrap();

    while b > 0 {
        if b % 2 == 1 {
            sum = sum * a % 10;
        };
        b /= 2;
        a = a * a % 10;
    };
    sum as i32
}

fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" { return 1; }
    let x = str1.chars().last().unwrap().to_digit(10).unwrap();
    let m = str2.chars().fold(0, |a,x| (a*10 + x.to_digit(10).unwrap()) % 4);
    let exp = if m == 0 { 4 } else { m };
    (x.pow(exp) % 10) as i32
}

#[test]
fn returns_expected() {
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10", "10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376", "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
}