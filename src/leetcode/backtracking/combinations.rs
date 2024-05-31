pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut solutions = vec![];
        let mut list = vec![];
        Self::solve(&mut list, n, k, 1, &mut solutions);
        solutions
    }

    fn solve(list: &mut Vec<i32>, n: i32, k: i32, start: i32, solutions: &mut Vec<Vec<i32>>) {
        if list.len() == k as usize {
            solutions.push(list.clone());
            return;
        }

        for i in start..=n {
            if let None = list.iter().find(|item| **item == i) {
                list.push(i);
                Self::solve(list, n, k, i + 1, solutions);
                list.pop();
            }
        }
    }
}
