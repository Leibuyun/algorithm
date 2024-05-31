struct Solution;

fn solve(
    digits_len: usize,
    containers: &mut Vec<Vec<char>>,
    cur_word: &mut Vec<char>,
    cur_choose_idx: usize,
    solutions: &mut Vec<String>,
) {
    if cur_word.len() == digits_len {
        solutions.push(cur_word.iter().collect());
        return;
    }
    if cur_choose_idx >= containers.len() {
        return;
    }
    let mut cp = vec![];
    while let Some(char) = containers[cur_choose_idx].pop() {
        cur_word.push(char);
        solve(
            digits_len,
            containers,
            cur_word,
            cur_choose_idx + 1,
            solutions,
        );
        let char = cur_word.pop().unwrap();
        cp.push(char);
    }
    containers[cur_choose_idx] = cp;
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = std::collections::HashMap::from([
            (2u32, vec!['a', 'b', 'c']),
            (3, vec!['d', 'e', 'f']),
            (4, vec!['g', 'h', 'i']),
            (5, vec!['j', 'k', 'l']),
            (6, vec!['m', 'n', 'o']),
            (7, vec!['p', 'q', 'r', 's']),
            (8, vec!['t', 'u', 'v']),
            (9, vec!['w', 'x', 'y', 'z']),
        ]);
        let mut solutions = vec![];
        let mut containers = vec![];
        digits.chars().into_iter().for_each(|char| {
            let number = char.to_digit(10).unwrap();
            let list = map.get(&number).unwrap().clone();
            containers.push(list);
        });
        let mut cur_word = vec![];
        if digits.len() > 0 {
            solve(
                digits.len(),
                &mut containers,
                &mut cur_word,
                0,
                &mut solutions,
            );
        }
        solutions
    }
}
