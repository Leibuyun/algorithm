struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut solutions = vec![];
        let mut parenthesis = String::new();
        Self::solve(&mut parenthesis, n as usize, 0, 0, &mut solutions);
        solutions
    }

    // 生成括号，当左括号 < n时，添加左括号。当右括号小于左括号时，添加右括号。
    pub fn solve(
        parenthesis: &mut String,
        n: usize,
        left_cnt: usize,
        right_cnt: usize,
        solutions: &mut Vec<String>,
    ) {
        if parenthesis.len() == 2 * n {
            solutions.push(parenthesis.clone());
            return;
        }
        if left_cnt < n {
            parenthesis.push('(');
            Self::solve(parenthesis, n, left_cnt + 1, right_cnt, solutions);
            parenthesis.pop();
        }
        if right_cnt < left_cnt {
            parenthesis.push(')');
            Self::solve(parenthesis, n, left_cnt, right_cnt + 1, solutions);
            parenthesis.pop();
        }
    }
}
