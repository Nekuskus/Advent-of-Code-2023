use setup_utils::*;
use std::{path::Path, collections::HashMap};

// Symbols to replace: 08 6 6 17621 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/08-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 2 {
            Ok(())
        } else {
            Err(format!("08: Bad result for Part 1 example, expected 6 got {}", result))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/08-2-example.txt"));
        let result = crate::part2(&lines);
        if result == 6 {
            Ok(())
        } else {
            Err(format!("08: Bad result for Part 2 example, expected 6 got {}", result))
        }
    }
    */
    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/08-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == 17621 {
            Ok(())
        } else {
            Err(format!("08: Bad result for Part 1, expected 17621 got {}", result1))
        }
        /*
        match (result1, result2) {
            (17621, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("08: Bad result for Part 1, expected 17621 got {}", result1)),
            (17621, _) => Err(format!("08: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("08: Bad result for Part 1 & 2, expected (17621, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/08-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/08-1-example.txt"));
    let lines2 = read_lines(Path::new("./inputs/08-2-example.txt"));

    println!("08-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("08-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("08-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}

#[derive(Clone, Debug)]
struct CamelNode {
    name: String,
    left: String,
    right: String
}


fn part1(lines: &Vec::<String>) -> i32 {
    let mut steps = lines[0].chars().cycle();
    let node_lines = lines[2..].to_vec();
    let mut nodes: HashMap<String, CamelNode> = HashMap::new();
    node_lines.iter().for_each(|s| {
        let name = &s[0..3].to_string();
        let left = &s[7..10].to_string();
        let right = &s[12..15].to_string();
        //println!("{name} ({left}, {right})")
        nodes.insert(name.clone(), CamelNode { name: name.clone(), left: left.clone(), right: right.clone()});
    });
    let mut total_steps = 0;
    let mut curnode = nodes.get("AAA").unwrap();
    while curnode.name != "ZZZ" {
        match steps.next() {
            Some(c) => { match c {
                'R' => curnode = nodes.get(&curnode.right).unwrap(),
                'L' => curnode = nodes.get(&curnode.left).unwrap(),
                _ => ()
                }
            },
            None => ()
        }
        total_steps += 1;
    }

    return total_steps;
}

fn part2(lines: &Vec::<String>) -> i32 {
    let mut steps = lines[0].chars().cycle();
    let node_lines = lines[2..].to_vec();
    let mut nodes: HashMap<String, CamelNode> = HashMap::new();
    node_lines.iter().for_each(|s| {
        let name = &s[0..3].to_string();
        let left = &s[7..10].to_string();
        let right = &s[12..15].to_string();
        //println!("{name} ({left}, {right})")
        nodes.insert(name.clone(), CamelNode { name: name.clone(), left: left.clone(), right: right.clone()});
    });

    
    let mut total_steps = 0;
    let mut curnodes = nodes.iter().filter(|n| n.1.name.ends_with("A")).map(|n| n.1.clone()).collect::<Vec<_>>();
    while !curnodes.iter().all(|n| n.name.ends_with("Z")) {
        println!("{curnodes:#?}");
        let next_step = steps.next().unwrap();
        curnodes = curnodes.iter().map(|n| {
            let node = &match next_step {
                    'R' => nodes.get(&n.right).unwrap(),
                    'L' => nodes.get(&n.left).unwrap(),
                    _ => return CamelNode { name: "-1".to_string(), right: "-1".to_string(), left: "-1".to_string() }
            }.clone();
            return node.clone();
        }).collect::<Vec<CamelNode>>();
        total_steps += 1;
    }

    return total_steps;
}
