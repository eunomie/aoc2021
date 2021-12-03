use std::{fs, isize};

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input file");
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
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

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut oxygen = lines.clone();
    let mut co2 = lines.clone();

    for i in 0..lines[0].len() {
        let oxygen_most_common = most_common_at(&oxygen, i);
        if oxygen.len() > 1 {
            oxygen = oxygen.into_iter().filter(|line| line.chars().nth(i).unwrap() == oxygen_most_common).collect();
        }
        let co2_most_common = most_common_at(&co2, i);
        if co2.len() > 1 {
            co2 = co2.into_iter().filter(|line| line.chars().nth(i).unwrap() != co2_most_common).collect();
        }
    }

    return i32::from_str_radix(oxygen[0], 2).unwrap() * i32::from_str_radix(co2[0], 2).unwrap();
}

fn most_common_at(lines: &Vec<&str>, i: usize) -> char {
    let nb_1 = lines.into_iter().filter(|line|
        line.chars().nth(i).unwrap() == '1'
    ).count();
    if 2 * nb_1 >= lines.len() {
        return '1';
    }
    return '0';
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

    #[test]
    fn test_part2() {
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

        assert_eq!(part2(input), 230);
    }
}
