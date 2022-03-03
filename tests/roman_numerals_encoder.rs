/**
 * @Author: ZZX
 * @Description: https://www.codewars.com/kata/51b62bf6a9c58071c600001b/train/rust
 * @Date: create in 2022/3/3 3:30 PM
 */

/// Converts a number to a string representating roman numeral.

fn num_as_roman(num: i32) -> String {
    let mut new_num = num.clone();
    let mut result = String::from("");
    let a = if new_num >= 1000 {
        let r = thousands((new_num / 1000) as i8, "M");
        new_num %= 1000;
        r
    } else {
        String::from("")
    };
    println!("{}", num);
    let b = if new_num >= 100 {
        let r = greater((new_num / 100) as i8, ["C", "D", "M"]);
        new_num %= 100;
        r
    } else {
        String::from("")
    };
    let c = if new_num >= 10 {
        let r = greater((new_num / 10) as i8, ["X", "L", "C"]);
        new_num %= 10;
        r
    } else {
        String::from("")
    };
    let d = if new_num >= 1 {
        greater((new_num / 1) as i8, ["I", "V", "X"])
    } else {
        String::from("")
    };
    println!("{:?},{:?},{:?},{:?}", a, b, c, d);
    result.push_str(a.as_str());
    result.push_str(b.as_str());
    result.push_str(c.as_str());
    result.push_str(d.as_str());
    result
}

fn greater(num: i8, arr: [&str; 3]) -> String {
    println!("{}", num);
    if num >= 1 && num <= 3 {
        thousands(num, arr[0])
    } else if num == 4 {
        let mut a = arr[0].to_string();
        println!("{}", &a);
        a.push_str(arr[1]);
        a
    } else if num >= 5 && num <= 8 {
        let mut a = arr[1].to_string();
        println!("{}", &a);
        a.push_str(&thousands(num - 5, arr[0]));
        a
    } else if num == 9 {
        let mut a = arr[0].to_string();
        println!("{}", &a);

        a.push_str(arr[2]);
        a
    } else {
        String::from("")
    }
}

fn thousands(num: i8, str: &str) -> String {
    let mut result = String::from("");
    for _i in 0..num {
        result.push_str(str)
    }
    result
}

#[test]
fn returns_expected() {
    assert_eq!(num_as_roman(182), "CLXXXII");
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}