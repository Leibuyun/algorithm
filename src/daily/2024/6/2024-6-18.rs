// Most Profit Assigning Work
impl Solution {
    // O(n^2)
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        for power in worker.iter() {
            let mut item_max = 0;
            for (idx, diff) in difficulty.iter().enumerate() {
                if *power >= *diff && profit[idx] > item_max {
                    item_max = profit[idx];
                }
            }
            max_profit += item_max;
        }
        max_profit
    }
}
