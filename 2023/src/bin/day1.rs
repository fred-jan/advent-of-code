use std::fs;

fn pt1_calibration_sum(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|entry| {
            let digits: Vec<u32> = entry
                .chars()
                .filter(|char| char.is_digit(10))
                .map(|char| char.to_digit(10).unwrap())
                .collect();

            digits.first().unwrap() * 10u32 + digits.last().unwrap()
        })
        .sum()
}

fn pt2_calibration_sum(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let digits: Vec<u32> = line
                .replace("oneight", "18")
                .replace("twone", "21")
                .replace("eightwo", "82")
                .replace("eighthree", "83")
                .replace("sevenine", "79")
                .replace("nineight", "98")
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
                .chars()
                .filter(|char| char.is_digit(10))
                .map(|char| char.to_digit(10).unwrap())
                .collect();

            digits.first().unwrap() * 10u32 + digits.last().unwrap()
        })
        .sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").expect("Failed to read input");
    let lines = contents.lines().map(|line| line.to_string());

    println!("Part 1: {}", pt1_calibration_sum(lines.clone().collect()));
    println!("Part 2: {}", pt2_calibration_sum(lines.clone().collect()));
}
