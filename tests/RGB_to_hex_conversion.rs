/**
 * @Author: BearV
 * @Description: 
 * @Date: create in 2022/3/8 2:39 PM
 */

fn _rgb(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}", check_num(r), check_num(g), check_num(b))
}

fn check_num(n: i32) -> i32 {
    match n {
        n if n < 0 => 0,
        n if n > 255 => 255,
        _ => n,
    }
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        r.clamp(0, 255),
        g.clamp(0, 255),
        b.clamp(0, 255)
    )
}

macro_rules! compare {
  ( $got : expr, $expected : expr ) => {
    if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
  };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
