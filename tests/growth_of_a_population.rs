/**
 * @Author: ZZX
 * @Description: https://www.codewars.com/kata/563b662a59afc2b5120000c6/train/rust
 * @Date: create in 2022/3/3 2:25 PM
 */
//In a small town the population is p0 = 1000 at the beginning of a year. The population regularly increases by 2 percent per year and moreover 50 new inhabitants per year come to live in the town. How many years does the town need to see its population greater or equal to p = 1200 inhabitants?
//
// At the end of the first year there will be:
// 1000 + 1000 * 0.02 + 50 => 1070 inhabitants
//
// At the end of the 2nd year there will be:
// 1070 + 1070 * 0.02 + 50 => 1141 inhabitants (** number of inhabitants is an integer **)
//
// At the end of the 3rd year there will be:
// 1141 + 1141 * 0.02 + 50 => 1213
//
// It will need 3 entire years.
// More generally given parameters:
//
// p0, percent, aug (inhabitants coming or leaving each year), p (population to surpass)
//
// the function nb_year should return n number of entire years needed to get a population greater or equal to p.
//
// aug is an integer, percent a positive or null floating number, p0 and p are positive integers (> 0)
//
// Examples:
// nb_year(1500, 5, 100, 5000) -> 15
// nb_year(1500000, 2.5, 10000, 2000000) -> 10
// Note:
// Don't forget to convert the percent parameter as a percentage in the body of your function: if the parameter percent is 2 you have to convert it to 0.02.

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    // your code

    next_year(p0, percent, aug, p, 1)
}

fn next_year(origin_p: i32, percent: f64, aug: i32, p: i32, count: i32) -> i32 {
    let next_g = origin_p as f64 * (percent / 100.0) + aug as f64;
    let next_p = origin_p + next_g as i32;
    println!("{},{},{},{}", next_p, next_g, (percent / 100.0), count);
    return if next_p > p {
        count
    } else {
        next_year(next_p, percent, aug, p, count + 1)
    };
}

fn _nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut pop = p0;
    let multiplier : f64 = 1 as f64 + percent/100.0;
    let mut counter = 0;
    while pop < p {
        pop = (pop as f64 * multiplier).floor() as i32 + aug;
        counter += 1;
    }

    counter
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans = nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
    }
}