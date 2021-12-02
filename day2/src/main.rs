use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input file");
    println!("part 1: {}", part1(&input));
}

fn part1(input: &str) -> i32 {
    let mut horiz: i32 = 0;
    let mut depth: i32 = 0;

    input.lines().for_each(|line| {
        let tokens: Vec<&str> = line.splitn(2, ' ').collect();
        let order = tokens[0];
        let v: i32 = tokens[1].trim().parse().expect("int expected");
        if order == "forward" {
            horiz += v;
        } else if order == "down" {
            depth += v;
        } else if order == "up" {
            depth -= v;
        }
    });
    return horiz * depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(part1(input), 150);
    }
}
