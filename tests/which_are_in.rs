use std::collections::HashSet;

/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/550554fd08b86f84fe000a58/train/rust
 * @Date: create in 2022/3/4 4:13 PM
 */
// 查找才b中出现过的 a 单词 返回值不重复
fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut v: HashSet<String> = HashSet::new();
    for b in arr_b {
        for a in arr_a {
            if b.contains(a) && !v.contains(&a.to_string()) {
                v.insert(a.to_string());
            }
        }
    }

    let mut result = v.into_iter().collect::<Vec<String>>();
    result.sort();
    result
}

// use itertools::Itertools;
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     arr_a
//         .iter()
//         .map(|x| x.to_string())
//         .filter(|x| { arr_b.into_iter().any(|y| y.contains(x.as_str()))} )
//         .unique()
//         .sorted()
//         .collect()
// }


fn _in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = arr_a.iter()
        .filter(|&e| arr_b.iter()
            .any(|&t| t.contains(e)))
        .map(|s| s.to_string())
        .collect();

    result.sort_unstable();
    result.dedup();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(in_array(
            &["xyz", "live", "strong"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["live", "strong"]);

        assert_eq!(in_array(
            &["live", "strong", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);

        assert_eq!(in_array(
            &["tarp", "mice", "bull"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), [] as [&str; 0]);

        assert_eq!(in_array(
            &["live", "strong", "arp", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);
    }
}
