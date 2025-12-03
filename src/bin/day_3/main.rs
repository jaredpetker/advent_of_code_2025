use advent_of_code_2025::include_file;
use std::str::FromStr;

struct Bank(Vec<u32>);

impl FromStr for Bank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .expect(&format!("Expected digit but received {}", c))
            })
            .collect::<Vec<_>>();

        Ok(Bank(digits))
    }
}

impl Bank {
    fn compute_joltage(&self, max_digits: usize) -> u128 {
        let mut joltage = vec![0; max_digits];
        let bank_len = self.0.len();

        for (battery_idx, rating) in self.0.iter().enumerate() {
            // minimum index where we can start altering the joltage from
            let min_joltage_alteration_idx = max_digits
                .saturating_sub(bank_len.saturating_sub(battery_idx))
                .clamp(0, max_digits);

            // if joltage at joltage index is less than the current rating
            // then update the joltage at the index to the current rating
            // and zero out the rest of the joltage
            for joltage_idx in min_joltage_alteration_idx..joltage.len() {
                if *rating > joltage[joltage_idx] {
                    joltage[joltage_idx] = *rating;
                    for j in (joltage_idx + 1)..joltage.len() {
                        joltage[j] = 0;
                    }
                    break;
                }
            }
        }

        joltage
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .parse::<u128>()
            .unwrap()
    }
}

fn main() {
    let input = include_file!("input");

    let joltage_pairs = input
        .lines()
        .map(|line| {
            Bank::from_str(line).expect(&format!("Expected digits for Bank but received: {}", line))
        })
        .map(|bank| (bank.compute_joltage(2), bank.compute_joltage(12)))
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", joltage_pairs.iter().map(|(max_digits_2, _)| max_digits_2).sum::<u128>());
    println!("Part 2: {:?}", joltage_pairs.iter().map(|(_, max_digits_12)| max_digits_12).sum::<u128>());
}