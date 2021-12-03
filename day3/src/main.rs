use std::{fs, isize};

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input file");
    println!("part 1: {}", part1(&input));
}

fn part1(input: &str) -> isize {
    let lines: Vec<&str> = input.lines().collect();
    let nb_lines = lines.len();
    let mid = nb_lines / 2;
    let line_len = lines[0].len();
    let mut counts: Vec<u32> = vec![0; line_len];
    for line in lines {
        let chars = line.chars().map(|c| c.to_digit(10).unwrap());
        for (i, c) in chars.enumerate() {
            counts[i] += c;
        }
    }

    let mut gamma_bin_arr: Vec<u32> = vec![0; line_len];
    let mut epsilon_bin_arr: Vec<u32> = vec![0; line_len];
    for i in 0..line_len {
        if counts[i] > (mid as u32) {
            gamma_bin_arr[i] = 1;
        } else {
            epsilon_bin_arr[i] = 1;
        }
    }
    let gamma_bin_str: String = gamma_bin_arr.iter().map(ToString::to_string).collect();
    let epsilon_bin_str: String = epsilon_bin_arr.iter().map(ToString::to_string).collect();

    let gamma = isize::from_str_radix(&gamma_bin_str, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_bin_str, 2).unwrap();

    return gamma * epsilon;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(part1(input), 198);
    }
}
