pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut solutions = vec![];
        let mut list = vec![];
        let mut used = vec![false; nums.len()];
        Self::solve(&mut list, &nums, &mut used, &mut solutions);
        solutions
    }

    fn solve(
        list: &mut Vec<i32>,
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
        solutions: &mut Vec<Vec<i32>>,
    ) {
        if list.len() == nums.len() {
            solutions.push(list.clone());
            return;
        }

        for (i, item) in nums.iter().enumerate() {
            if used[i] {
                continue;
            }
            list.push(*item);
            used[i] = true;
            Self::solve(list, nums, used, solutions);
            list.pop();
            used[i] = false;
        }
    }
}
