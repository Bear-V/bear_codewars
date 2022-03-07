/**
 * @Author: BearV
 * @Description: 计算阶乘末尾0数量
 * 原文链接：https://blog.csdn.net/u011544909/article/details/79810245
    输入一个正整数n,求n!(即阶乘)末尾有多少个0？ 比如: n = 10; n! = 3628800,所以答案为2。

    思路：末尾0的个数就是指这个数总共有几个10因子，而10又能表示成2和5的乘积。假设m=n!，那么m中2的因子个数肯定大于5的因子个数，所以m中5的因子个数即是所要求结果；

    显然n除以5可得到1~n中包含有一个因子5的个数，但是，1~n中有的数可以被5整除好几次，所以必须将这个数再除以5，得到1~n中包含有两个因子5的个数，依次循环进行累加即可得到全部5的因子个数；
* @Date: create in 2022/3/4 2:47 PM
 */

fn zeros(n: u64) -> u64 {
    let mut new_n = n;
    let mut count = 0;
    while new_n >= 5 {
        new_n = new_n / 5;
        count += new_n;
    }
    count
}

mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}