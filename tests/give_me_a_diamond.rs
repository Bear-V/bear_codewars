/**
 * @Author: ZZX
 * @Description: 
 * @Date: create in 2022/3/4 10:24 AM
 */
///You need to return a string that looks like a diamond shape when printed on the screen, using asterisk (*) characters. Trailing spaces should be removed, and every line must be terminated with a newline character (\n).
//
// Return null/nil/None/... if the input is an even number or negative, as it is not possible to print a diamond of even or negative size.

fn print(n: i32) -> Option<String> {
    if n <= 0 || n % 2 == 0 {
        return None;
    }

    let mut result = String::new();
    let mut v = vec![(0, 0); n as usize];
    let mid_row = ((n as f64 / 2.0).floor() + 1.0) as i32;
    println!("mid_row: {}", mid_row);
    for i in 0..mid_row {
        let size = 1 + (i * 2);
        let space = (n - size) / 2;
        v[i as usize] = (space, size);
        if i != mid_row {
            v[((n - i) - 1) as usize] = (space, size);
        }
    }

    for (space, size) in v {
        let mut row = String::new();

        for _sp in 0..space {
            row.push_str(" ");
        }
        for s in 0..size {
            if s == size-1 {
                row.push_str("*\n");
            } else {
                row.push_str("*");
            }
        }
        result.push_str(&row)
    }


    Some(result)
}


fn _print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }

    let n = n as usize;
    let diamond = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
        .collect();

    Some(diamond)
}

#[test]
fn basic_test() {
    // assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}