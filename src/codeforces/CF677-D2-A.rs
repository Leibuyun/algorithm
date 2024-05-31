fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let nums: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (n, h) = (nums[0], nums[1]);

    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s2).unwrap();
    let lst: Vec<usize> = s2.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let val: usize = lst.iter().map(|item| if *item > h { 2 } else { 1 }).sum();
    println!("{}", val);
}
