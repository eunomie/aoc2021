use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input file");
    let count = count_increase(&input);
    println!("depth increased {} times", count)
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
}
