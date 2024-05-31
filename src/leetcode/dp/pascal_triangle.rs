struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 1..=num_rows {
            let mut row = vec![1; i as usize];
            for j in 1..i - 1 {
                row[j] = ans[i - 2][j] + ans[i - 2][j - 1];
            }
            ans.push(row);
        }
        ans
    }
}
