use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
fn main() {
    let mut linesfull = read_lines("inputs\\01-full.txt");
    let mut lines1 = read_lines("inputs\\01-1-example.txt");
    let mut lines2 = read_lines("inputs\\01-2-example.txt");
    for i in 0..linesfull.len() {
        linesfull[i] = linesfull[i].trim().to_owned();
    }
    for i in 0..lines1.len() {
        lines1[i] = lines1[i].trim().to_owned();
    }
    
    for i in 0..lines2.len() {
        lines2[i] = lines2[i].trim().to_owned();
    }

    println!("01-full.txt");
    println!("{}", first(&linesfull));
    println!("{}", second(&linesfull));
    
    println!("01-1-example.txt");
    println!("{}", first(&lines1));
    println!("{}", second(&lines1));
    
    println!("01-2-example.txt");
    println!("{}", first(&lines2));
    println!("{}", second(&lines2));
    
}

fn first(lines: &Vec::<String>) -> i32 {
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
        if sum != "" {
            nums.push(sum.parse::<i32>().unwrap());
        }
    }
    let sum = nums.iter().sum::<i32>();
    
    return sum
}

// one, two, three, four, five, six, seven, eight, nine
fn second(lines: &Vec::<String>) -> i32 {
    let mut sums = Vec::new();
    for line in lines {
        let mut first = String::new();
        let mut last = String::new();
        let chars = line.chars().collect::<Vec<char>>();
        for i in 0..line.len() {
            if chars[i].is_numeric() {
                if first == "" {
                    first = chars[i].to_string();
                }
                last = chars[i].to_string();
            } else {
                let mut slice = chars[i..i].iter();
                if i+5 <= chars.len() {
                    slice = chars[i..i+5].iter();
                } else if i+4 <= chars.len() {
                    slice = chars[i..i+4].iter();
                } else if i+3 <= chars.len() {
                    slice = chars[i..i+3].iter();
                }
                let potential_num = String::from_iter(slice);
                let found_num = match potential_num.as_str() {
                    _ if potential_num.starts_with("one") => "1",
                    _ if potential_num.starts_with("two") => "2",
                    _ if potential_num.starts_with("three") => "3",
                    _ if potential_num.starts_with("four") => "4",
                    _ if potential_num.starts_with("five") => "5",
                    _ if potential_num.starts_with("six") => "6",
                    _ if potential_num.starts_with("seven") => "7",
                    _ if potential_num.starts_with("eight") => "8",
                    _ if potential_num.starts_with("nine") => "9",
                    _ => "-1"
                }.to_owned();
                if first == "" && found_num != "-1" {
                    //println!("{}: {}", potential_num, found_num);
                    first = found_num.to_owned();
                }
                if found_num != "-1" {
                    last = found_num;
                }
            }
        }
        //println!("{}: {} {}", line, first, last);
        let sum = first + &last;
        sums.push(sum);
    }
    let mut nums = Vec::new();
    for sum in sums {
        nums.push(sum.parse::<i32>().unwrap());
    }
    let sum = nums.iter().sum::<i32>();
    
    return sum
}