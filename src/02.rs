use setup_utils::read_lines;
use std::path::Path;
use std::collections::HashMap;


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/02-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 8 {
            Ok(())
        } else {
            Err(format!("01: Bad result for Part 1 example, expected 8 got {}", result))
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/02-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/02-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/02-2-example.txt"));

    println!("01-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("01-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("01-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}

fn part1(lines: &Vec::<String>) -> i32 {
    let mut sum_of_ids = 0;
    for i in 0..lines.len() {
        let line_id = i as i32 + 1;
        let mut line_record: HashMap<String, i32> = HashMap::new();
        let line_of_balls = lines[i].split(' ').map(String::from).collect::<Vec<String>>()[2..].join(" ");
        let replaced_line = line_of_balls.replace(";", &",");
        let arr_of_balls = replaced_line.split(",").map(|s| s.trim().split(' ')).map(|spl| spl.collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        for arr in arr_of_balls {
            //println!("{}, {}", arr[0], arr[1]);
            let amount: i32 = arr[0].parse().unwrap();
            let color = arr[1];
            if !line_record.contains_key(color) {
                line_record.insert(color.to_owned(), amount);
            } else if line_record.get(color).unwrap() < &amount {
                *line_record.get_mut(color).unwrap() = amount;
            }
        }
        if  line_record.get("red").unwrap_or(&99) <= &12
            && line_record.get("green").unwrap_or(&99) <= &13 
            && line_record.get("blue").unwrap_or(&99) <= &14  {
            //println!("id: {}, red: {}, green: {}, blue: {}", line_id, line_record.get("red").unwrap(), line_record.get("green").unwrap(), line_record.get("blue").unwrap());
            sum_of_ids += line_id;
        }
    }
    return sum_of_ids;
}