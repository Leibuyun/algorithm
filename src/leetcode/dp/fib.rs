struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            n
        } else {
            Self::fib(n - 2) + Self::fib(n - 1)
        }
    }
}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            n
        } else {
            let (mut a, mut b) = (0, 1);
            for _ in 2..=n {
                let sum = a + b;
                a = b;
                b = sum;
            }
            b
        }
    }
}
