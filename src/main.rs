struct Solution;

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats;
        seats.sort();
        let mut students = students;
        students.sort();
        seats
            .iter()
            .zip(students.iter())
            .map(|(x, y)| (*x - *y).abs())
            .sum()
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_moves_to_seat(vec![12, 14, 19, 19, 12], vec![19, 2, 17, 20, 7])
    )
}
