use setup_utils::*;
use std::path::Path;

// Symbols to replace: 11 374 8410 9623138 726820169514


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/11-example.txt"));
        let result = crate::part1(&lines);
        if result == 374 {
            Ok(())
        } else {
            Err(format!("11: Bad result for Part 1 example, expected 374 got {}", result))
        }
    }
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/11-example.txt"));
        let result = crate::part2(&lines, 100);
        if result == 8410 {
            Ok(())
        } else {
            Err(format!("11: Bad result for Part 2 example, expected 8410 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/11-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines, 1000000);

        match (result1, result2) {
            (9623138, 726820169514) => Ok(()),
            (_, 726820169514) => Err(format!("11: Bad result for Part 1, expected 9623138 got {}", result1)),
            (9623138, _) => Err(format!("11: Bad result for Part 2, expected 726820169514 got {}", result2)),
            (_, _) => Err(format!("11: Bad result for Part 1 & 2, expected (9623138, 726820169514) got ({}, {})", result1, result2))
        }
    }
    
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/11-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/11-1-example.txt"));

    println!("11-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull, 1000000));
    
    println!("11-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1, 1000000));
    
}


fn part1(lines: &Vec<String>) -> i32 {
    let mut expanded: Vec<Vec<char>> = Vec::new();
    for line in lines {
        expanded.push(line.chars().collect());
    }
    let mut x = 0;
    while x < expanded[0].len() {    
        if expanded.iter().all(|s| s[x] == '.') {
            expanded.iter_mut().for_each(|s| {
                s.insert(x, '.');
            });
            x += 1;
        }
        x += 1;
    }

    let mut y = 0;
    while y < expanded.len() { 
        if expanded[y].iter().all(|c| c == &'.') {
            expanded.insert(y, expanded[y].clone());
            y += 1;
        }
        y += 1;
    }

    let mut stars = vec![];
    let mut i = 1;
    for x in 0..expanded.len() {
        for y in 0..expanded[x].len() {
            if expanded[x][y] == '#' {
                stars.push((x as i32, y as i32, i));
                i += 1;
            }
        }
    }

    let mut cumsum = 0;

    for p1 in &stars {
        for p2 in &stars {
            if p2.2 > p1.2 {
                let diff = (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs();
                //println!("Dist between galaxies {} and {}: {diff}", p1.2, p2.2);
                cumsum += diff
            }
        }
    }

    return cumsum;
}

fn part2(lines: &Vec<String>, num_of_expansion: u128) -> u128 {
    let mut expansion_x = vec![];
    let mut expansion_y = vec![];
    
    for x in 0..lines[0].len() {    
        if lines.iter().all(|s| s.chars().nth(x).unwrap() == '.') {
            expansion_x.push(x as i32);
        }
    }

    for y in 0..lines.len() { 
        if lines[y].chars().all(|c| c == '.') {
            expansion_y.push(y as i32);
        }
    }


    let mut stars = vec![];
    let mut i = 1;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y].chars().nth(x).unwrap() == '#' {
                stars.push((x as i32, y as i32, i));
                i += 1;
            }
        }
    }

    let mut cumsum = 0;

    for p1 in &stars {
        for p2 in &stars {
            if p2.2 > p1.2 {
                let mut tx = p1.0.min(p2.0);
                let mut ty = p1.1.min(p2.1);
                
                let xdest = p1.0.max(p2.0);
                let ydest = p1.1.max(p2.1);

                let mut total_steps: u128 = 0;

                while tx < xdest {
                    tx += 1;
                    total_steps += if expansion_x.contains(&tx) { num_of_expansion } else { 1 };
                }

                while ty < ydest {
                    ty += 1;
                    total_steps += if expansion_y.contains(&ty) { num_of_expansion } else { 1 };
                }
                

                
                //println!("Dist between galaxies {} and {}: {diff}", p1.2, p2.2);
                cumsum += total_steps
            }
        }
    }
    return cumsum;
}
