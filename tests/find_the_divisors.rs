// Create a function named divisors/Divisors that takes an integer n > 1 and returns an array with all of the integer's divisors(except for 1 and the number itself), from smallest to largest. If the number is prime return the string '(integer) is prime' (null in C#) (use Either String a in Haskell and Result<Vec<u32>, String> in Rust).
// 创建一个名为 divisors/Divisors 的函数，
// 它接受一个整数 n > 1 并返回一个包含所有整数除数（除了 1 和数字本身）的数组，从小到大。
//  如果数字是素数，则返回字符串 '(integer) is prime'（在 C# 中为 null）
// （在 Haskell 中使用 Either String a，在 Rust 中使用 Result<Vec<u32>, String>）。

fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut arr: Vec<u32> = Vec::new();
    for a in 2..integer {
        if integer % a == 0 {
            arr.push(a)
        }
    }
    if arr.len() == 0 {
       return Err(format!("{} is prime", integer))
    }
    Ok(arr)
}

fn _divisors(integer: u32) -> Result<Vec<u32>, String> {
    let divs = (2..integer)
        .filter(|k| integer % k == 0)
        .collect::<Vec<u32>>();

    if divs.len() > 0 {
        Ok(divs)
    } else {
        Err(format!("{} is prime", integer))
    }
}

#[test]
fn tests() {
    assert_eq!(divisors(15), Ok(vec![3, 5]));
    assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
    assert_eq!(divisors(13), Err("13 is prime".to_string()));
}
