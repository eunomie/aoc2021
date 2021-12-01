use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input file");
    let count = count_increase(&input);
    println!("depth increased {} times", count);

    let count_windows = count_sliding(&input);
    println!("depth increased using sliding windows {} times", count_windows);
}

fn count_increase(input: &str) -> u32 {
    let mut prev = u32::MAX;
    let mut count: u32= 0;
    for line in input.lines() {
        let depth: u32 = line.trim().parse().expect("unable to parse line");
        if depth > prev {
            count += 1;
        }
        prev = depth;
    }
    return count;
}

fn count_sliding(input: &str) -> u32 {
    let lines = input.lines();
    let depths: Vec<u32> = lines.map(|line| line.trim().parse().expect("unable to parse line")).collect();
    let mut count: u32= 0;
    for i in 3..depths.len() {
        if depths[i-2] + depths[i-1] + depths[i] > depths[i-3] + depths[i-2] + depths[i-1] {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increase() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(count_increase(input), 7)
    }

    #[test]
    fn test_count_sliding() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(count_sliding(input), 5)
    }
}
