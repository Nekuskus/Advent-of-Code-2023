use setup_utils::*;
use std::path::Path;
use std::thread;

// Symbols to replace: 05 35 46 278755257 26829166

#[derive(Clone, Debug)]
struct RangeMap {
    from_start: u64,
    to_start: u64,
    length: u64
}

#[allow(dead_code)]
impl RangeMap
{
    pub fn new(from_start: u64, to_start: u64, length: u64) -> RangeMap {
        return RangeMap {
            from_start: from_start,
            to_start: to_start,
            length: length
        };
    }

    #[inline(always)]
    pub fn contains_from(&self, value: &u64) -> bool {
        self.from_start <= *value && *value < self.from_start + self.length + 1
    }

    #[inline(always)]
    pub fn contains_to(&self, value: &u64) -> bool {
        self.to_start <= *value && *value < self.to_start + self.length + 1
    }
    
    #[inline(always)]
    pub fn map(&self, value: &u64) -> u64 {
        assert!(self.contains_from(value));
        *value + self.to_start - self.from_start
    }
}

impl PartialEq for RangeMap {
    fn eq(&self, other: &Self) -> bool {
        (self.from_start == other.from_start) && (self.to_start == other.to_start) && (self.length == other.length)
    }
}

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-example.txt"));
        let result = crate::part1(&lines);
        if result == 35 {
            Ok(())
        } else {
            Err(format!(
                "05: Bad result for Part 1 example, expected 35 got {}",
                result
            ))
        }
    }
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-example.txt"));
        let result = crate::part2(&lines);
        if result == 46 {
            Ok(())
        } else {
            Err(format!(
                "05: Bad result for Part 2 example, expected 46 got {}",
                result
            ))
        }
    }
    /*
    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (278755257, 26829166) => Ok(()),
            (_, 26829166) => Err(format!(
                "05: Bad result for Part 1, expected 278755257 got {}",
                result1
            )),
            (278755257, _) => Err(format!(
                "05: Bad result for Part 2, expected 26829166 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "05: Bad result for Part 1 & 2, expected (278755257, 26829166) got ({}, {})",
                result1, result2
            )),
        }
    }*/
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/05-full.txt"));
    let lines = read_lines(Path::new("./inputs/05-example.txt"));

    //println!("05-full.txt");
    //println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));

    println!("05-example.txt");
    println!("{}", part1(&lines));
    println!("{}\n", part2(&lines));
}

fn part1(lines: &Vec<String>) -> u64 {
    let seeds = lines[0].split(':').map(|s| s.trim()).collect::<Vec<_>>()[1]
        .split(' ')
        .map(|s| {
            s.parse::<u64>()
                .expect(&format!("Number was incorrect, number: {s}"))
        })
        .collect::<Vec<_>>();
    //println!("{seeds:?}");
    //first split categories
    let lines_categories = lines[2..].to_vec();
    let maps = lines_categories
        .split(|s| s == "")
        .map(|s| s.to_vec())
        .collect::<Vec<Vec<String>>>();
    let mut mappings: Vec<Vec<RangeMap>> = Vec::new();
    //println!("{maps:#?}");
    for map in maps {
        let _header = &map[0];
        let mapset = map[1..]
            .iter()
            .map(|s| {
                let split = s.split(' ').collect::<Vec<_>>();
                let dest_range_start = split[0].parse::<u64>().unwrap();
                let source_range_start = split[1].parse::<u64>().unwrap();
                let range_len = split[2].parse::<u64>().unwrap();
                //println!("({}, {}, {})", source_range_start, dest_range_start, range_len);
                RangeMap::new(
                    source_range_start, dest_range_start,
                    range_len
                )
                //println!("[{}..{}]:[{}..{}]", r.from_start, r.from_start + r.length, r.to_start, r.to_start + r.length);
            })
            .collect::<Vec<_>>();
        mappings.push(mapset);
    }
    let result = seeds
        .iter()
        .map(|seed| {
            let mut current_val = seed.clone();
            //print!("{current_val}");
            for mapset in &mappings {
                let mut found = false;
                let mut mapped: u64 = 0;
                for rangemap in mapset {
                    if rangemap.contains_from(&current_val) {
                        mapped = rangemap.map(&current_val);
                        found = true;
                        break;
                    }
                }
                if !found {
                    mapped = current_val;
                }
                current_val = mapped;
                //print!(" -> {current_val}");
            }
            //println!("");
            current_val
        })
        .collect::<Vec<_>>();

    //println!("{result:?}");

    return result.iter().min().unwrap().clone();
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut lowest = u64::MAX;
    let _ = thread::scope(|s| {
    let seeds_unparsed = lines[0].split(':').map(|s| s.trim()).collect::<Vec<_>>()[1]
        .split(' ')
        .map(|s| {
            s.parse::<u64>()
                .expect(&format!("Number was incorrect, number: {s}"))
        })
        .collect::<Vec<_>>();
    let seed_ranges = seeds_unparsed
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1] + 1);
    //println!("{seeds:?}");
    //first split categories
    let lines_categories = lines[2..].to_vec();
    let maps = lines_categories
        .split(|s| s == "")
        .map(|s| s.to_vec())
        .collect::<Vec<Vec<String>>>();
    let mut mappings = Vec::new();
    //println!("{maps:#?}");
    for map in maps {
        let _header = &map[0];
        let mapset = map[1..]
            .iter()
            .map(|s| {
                let split = s.split(' ').collect::<Vec<_>>();
                let dest_range_start = split[0].parse::<u64>().unwrap();
                let source_range_start = split[1].parse::<u64>().unwrap();
                let range_len = split[2].parse::<u64>().unwrap();
                //println!("{split:?}: {}..{}, {}..{}", source_range_start, source_range_start+range_len, dest_range_start, dest_range_start+range_len);
                let r = RangeMap::new(source_range_start, dest_range_start, range_len);
                //println!("[{}..{}]:[{}..{}]", r.from_start, r.from_start + r.length, r.to_start, r.to_start + r.length);
                return r;
            })
            .collect::<Vec<_>>();
        mappings.push(mapset);
    }
        let join_vec = seed_ranges
            .map(|seed_range| {
                let mappings = mappings.clone();
                s.spawn(move || {
                    let mut num_of_iters: u64 = 0;
                    let mut valid_maps: Vec<RangeMap> = vec![];
                    let mut starts = vec![];
                    let mut straights: Vec<u64> = vec![];
                    println!("Seed range: [{}..{}]", seed_range.start, seed_range.end);
                    {
                        for seed in seed_range.clone() {
                            let mut found_a_map = false;
                            for map in mappings[0].clone() {
                                if map.contains_from(&seed) {
                                    if !valid_maps.contains(&map) {
                                        let cloned = map.clone();
                                        //println!("[{}..{}]:[{}..{}] -> {:?}", cloned.from_start, cloned.from_start + cloned.length, cloned.to_start, cloned.to_start + cloned.length, cloned.map(&seed));
                                        //println!("{cloned:?} -> {}", cloned.map(&seed));
                                        valid_maps.push(cloned);
                                        starts.push(seed);
                                    }
                                    found_a_map = true;
                                }
                                num_of_iters += 1;
                            }
                            if !found_a_map {
                                straights.push(seed);
                            }
                        }
                    }
                    for mapset in &mappings.clone()[1..] {
                        let cloned = valid_maps.clone();
                        let new_starts = cloned.iter().zip(starts).map(|(map, start)| map.map(&start)).chain(straights.clone());
                        valid_maps = vec![];
                        starts = vec![];
                        straights = vec![];
                        {
                            for seed in new_starts.clone() {
                                let mut found_a_map = false;
                                for map in mapset {
                                    if map.contains_from(&seed) {
                                        if !valid_maps.contains(&map) {
                                            let cloned = map.clone();
                                            //println!("[{}..{}]:[{}..{}] -> {:?}", cloned.from_start, cloned.from_start + cloned.length, cloned.to_start, cloned.to_start + cloned.length, cloned.map(&seed));
                                            valid_maps.push(cloned);
                                            starts.push(seed);
                                        }
                                        found_a_map = true;
                                    }
                                    num_of_iters += 1;
                                }
                                if !found_a_map {
                                    straights.push(seed);
                                }
                            }
                            println!("{:?}, {straights:?}", new_starts.collect::<Vec<_>>());
                        }
                    }
                    //println!("{starts:?}");
                    //(valid_maps.iter().zip(starts).map(|(map, start)| map.map(&start)).chain(straights).min().unwrap(), num_of_iters)
                    let lowest = starts.iter().chain(&straights).min().unwrap();
                    (lowest.clone(), num_of_iters)
                    
                    /*
                    return (seed_range
                        .map(|seed| {

                            
                            let mut current_val = seed.clone();
                            //print!("{current_val}");
                            for mapset in &mappings {
                                let mut found = false;
                                let mut mapped: u64 = 0;
                                for rangemap in mapset {
                                    if rangemap.contains_from(&current_val) {
                                        mapped = rangemap.map(&current_val);
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    mapped = current_val;
                                }
                                current_val = mapped;
                                //print!(" -> {current_val}");
                                num_of_iters += 1;
                            }
                            //println!("");
                            current_val
                        })
                        .min()
                        .unwrap(), num_of_iters);
                    */
                })
            })
            .collect::<Vec<_>>();
        let mut i = 0;
        let mut num_of_iters = 0;
        for jh in join_vec {
            let val = jh.join().unwrap();
            if val.0 < lowest {
                lowest = val.0;
            }
            println!("Thread {i} returned, count of iterations performed: {}", val.1);
            num_of_iters += val.1;
            i += 1;
        }
        println!("Iters: {num_of_iters}");
    });
    return lowest;
    //return result.iter().min().unwrap().clone();
}
