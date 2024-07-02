// Sum of Square Numbers
struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        if c == 0 {
            return true;
        }
        let sqrtc = (c as f64).sqrt().floor() as usize;
        for i in (1..=sqrtc).rev() {
            let diff = c as usize - i * i;
            let sqrt_diff = (diff as f64).sqrt();
            if sqrt_diff == sqrt_diff.floor() {
                return true;
            }
        }
        false
    }
}
