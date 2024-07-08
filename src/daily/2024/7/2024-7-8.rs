struct Solution;

// 约瑟夫闭环问题
fn helper(n: i32, k: i32) -> i32 {
    if n == 1 {
        0
    } else {
        (helper(n - 1, k) + k) % n
    }
}

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        helper(n, k) + 1
    }
}
