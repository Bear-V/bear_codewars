/**
 * @Author: BearV
 * @Description: https://leetcode-cn.com/problems/binary-search/
 * @Date: create in 2022/3/8 1:21 PM
 */


pub fn _search(nums: Vec<i32>, target: i32) -> i32 {
    let i = nums.iter().position(|x| {
        x == &target
    });
    match i {
        Some(i) => i as i32,
        None => -1,
    }
}


pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let i = nums.iter().position(|x| {
        x == &target
    });
    match i {
        Some(i) => i as i32,
        None => -1,
    }
}

#[test]
fn one() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4)
}