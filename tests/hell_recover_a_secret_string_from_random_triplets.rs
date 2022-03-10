/**
 * @Author: BearV
 * @Description: https://www.codewars.com/kata/53f40dff5f9d31b813000774/train/rust
 * @Date: create in 2022/3/10 3:46 PM
 */

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    String::new()
}


#[test]
fn example_test() {
    assert_eq!(recover_secret(vec![
        ['t', 'u', 'p'],
        ['w', 'h', 'i'],
        ['t', 's', 'u'],
        ['a', 't', 's'],
        ['h', 'a', 'p'],
        ['t', 'i', 's'],
        ['w', 'h', 's']])
               , "whatisup");
}
