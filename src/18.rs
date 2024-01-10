use std::collections::HashSet;
use setup_utils::*;
use std::{path::Path, collections::VecDeque};
use debug_print::{debug_print as debug, debug_println as debugln};

// Symbols to replace: 18 62 TEST2 SOLVE1 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/18-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 62 {
            Ok(())
        } else {
            Err(format!("18: Bad result for Part 1 example, expected 62 got {}", result))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/18-2-example.txt"));
        let result = crate::part2(&lines);
        if result == TEST2 {
            Ok(())
        } else {
            Err(format!("18: Bad result for Part 2 example, expected TEST2 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/18-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == SOLVE1 {
            Ok(())
        } else {
            Err(format!("18: Bad result for Part 1, expected SOLVE1 got {}", result1))
        }
        /*
        match (result1, result2) {
            (SOLVE1, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("18: Bad result for Part 1, expected SOLVE1 got {}", result1)),
            (SOLVE1, _) => Err(format!("18: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("18: Bad result for Part 1 & 2, expected (SOLVE1, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    */
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/18-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/18-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/18-2-example.txt"));

    println!("18-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("18-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("18-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}

// Up, Down, Right, Left
fn part1(lines: &Vec<String>) -> i32 {
    let mut grid = VecDeque::from([VecDeque::from([false; 1]); 1]);
    let start = Point::new(0, 0);
    let mut cur = start.clone();
    grid[start.y][start.x] = true;

    for line in lines {
        let strs: Vec<&str> = line.split_ascii_whitespace().collect();
        let count: i32 = strs[1].parse().unwrap();

        match strs[0] {
            "U" => {
                for i in 0..count {
                    if cur.y == 0 {
                        for _ in i..count { 
                            grid.push_front(VecDeque::from(vec![false].repeat(grid[0].len())));
                        }
                        cur.y += (count - i) as usize;
                    }
                    cur.y -= 1;
                    grid[cur.y][cur.x] = true;
                }
            },
            "D" => {
                for i in 0..count {
                    if cur.y == grid.len() - 1 {
                        for _ in i..count { 
                            grid.push_back(VecDeque::from(vec![false].repeat(grid[0].len())));
                        }
                    }
                    cur.y += 1;
                    grid[cur.y][cur.x] = true;
                }
            },
            "R" => {
                for i in 0..count {
                    if cur.x == grid[0].len() - 1 {
                        for _ in i..count { 
                            grid.iter_mut().for_each(|line| {
                                line.push_back(false)
                            });
                        }
                    }
                    cur.x += 1;
                    grid[cur.y][cur.x] = true;
                }
            },
            "L" => {
                for i in 0..count {
                    if cur.x == 0 {
                        for _ in i..count { 
                            grid.iter_mut().for_each(|line| {
                                line.push_front(false)
                            });
                        }
                        cur.x += (count - i) as usize;
                    }
                    cur.x -= 1;
                    grid[cur.y][cur.x] = true;
                }
            },
            _ => unreachable!()
        }
    }
    
    // Add outer layer for BFS to properly cover the area, use (0, 0)
    grid.push_front(VecDeque::from(vec![false].repeat(grid[0].len())));
    grid.push_back(VecDeque::from(vec![false].repeat(grid[0].len())));
    grid.iter_mut().for_each(|line| {
        line.push_front(false);
        line.push_back(false);
    });

    // BFS time!
    let mut visited: HashSet<Point> = HashSet::new();
    let mut to_visit = VecDeque::from(vec![start]);
    let mut bfs_markings = vec![vec![false; grid[0].len()]; grid.len()];


    while let Some(p) = to_visit.pop_front() {
        if visited.contains(&p) || grid[p.y][p.x] {
            continue;
        }

        if p.x > 0 {
            to_visit.push_back(Point::new(p.x - 1, p.y))
        }
        if p.x < grid[0].len() - 1 {
            to_visit.push_back(Point::new(p.x + 1, p.y))
        }
        if p.y > 0 {
            to_visit.push_back(Point::new(p.x, p.y - 1))
        }
        if p.y < grid.len() - 1 {
            to_visit.push_back(Point::new(p.x, p.y + 1))
        }
        bfs_markings[p.y][p.x] = true;
        visited.insert(p);
    }

    // Count border cells
    let perimeter_len = grid.iter().flatten().filter(|b| **b).count() as i32;
    let mut inner_count = 0;
    grid.iter_mut().enumerate().for_each(|(y, line)| {
        line.iter_mut().enumerate().for_each(|(x, item)| {
            if !*item && !bfs_markings[y][x] {
                inner_count += 1;
                *item = true;
            }
        })
    });

    

    for line in grid {
        debugln!("{}", line.iter().map(|&b| if b {'#'} else {'.'}).collect::<String>())
    }

    return perimeter_len +  inner_count;
}
/*
fn part2(lines: &Vec<String>) -> i32 {

}
*/