/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/56dbeec613c2f63be4000be6/train/rust
 * @Date: create in 2022/3/11 3:36 PM
 */

fn diag_1_sym(s: &str) -> String {
    // your code
    let rows: Vec<&str> = s.split('\n').collect();
    (0..rows.len())
        .map(|i| rows
            .iter()
            .map(|r| r
                .chars()
                .nth(i)
                .unwrap())
            .collect::<String>()
        )
        .collect::<Vec<_>>()
        .join("\n")
}

fn rot_90_clock(s: &str) -> String {
    diag_1_sym(s)
        .split('\n')
        .map(|r| r
            .chars()
            .rev()
            .collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn selfie_and_diag1(s: &str) -> String {
    s.split('\n')
        .zip(diag_1_sym(s)
            .split('\n')
        )
        .map(|(a, b)| a
            .to_owned() + "|" + b
        )
        .collect::<Vec<_>>().join("\n")
}

// first parameter: dots have to be replaced by function of one variable
fn oper<F>(f: F, s: &str) -> String
    where
        F: Fn(&str) -> String
{
    f(s)
}

fn testing1(s: &str, exp: &str) -> () {
    assert_eq!(oper(diag_1_sym, s), exp.to_string())
}

fn testing2(s: &str, exp: &str) -> () {
    assert_eq!(oper(rot_90_clock, s), exp.to_string())
}

fn testing3(s: &str, exp: &str) -> () {
    assert_eq!(oper(selfie_and_diag1, s), exp.to_string())
}

#[test]
fn basics_oper() {
    testing1("wuUyPC\neNHWxw\nehifmi\ntBTlFI\nvWNpdv\nIFkGjZ",
             "weetvI\nuNhBWF\nUHiTNk\nyWflpG\nPxmFdj\nCwiIvZ");

    testing2("rgavce\nvGcEKl\ndChZVW\nxNWgXR\niJBYDO\nSdmEKb",
             "Sixdvr\ndJNCGg\nmBWhca\nEYgZEv\nKDXVKc\nbORWle");

    testing3("NJVGhr\nMObsvw\ntPhCtl\nsoEnhi\nrtQRLK\nzjliWg",
             "NJVGhr|NMtsrz\nMObsvw|JOPotj\ntPhCtl|VbhEQl\nsoEnhi|GsCnRi\nrtQRLK|hvthLW\nzjliWg|rwliKg");
}