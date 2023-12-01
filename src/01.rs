use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
fn main() {
    let mut lines = read_lines("inputs\\01.txt");
    for i in 0..lines.len() {
        lines[i] = lines[i].trim().to_owned();
    }
    let mut sums = Vec::new();
    for line in lines {
        let mut first = String::new();
        let mut last = String::new();
        for c in line.chars() {
            if !c.is_numeric() {
                continue;
            }
            if first == "" {
                first = c.to_string();
            }
            last = c.to_string();
        }
        let sum = first + &last;
        sums.push(sum);
    }
    let mut nums = Vec::new();
    for sum in sums {
        nums.push(sum.parse::<i32>().unwrap());
    }
    let sum = nums.iter().sum::<i32>();
    println!("{}", sum)
}
