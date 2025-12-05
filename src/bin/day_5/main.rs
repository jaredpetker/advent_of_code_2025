use advent_of_code_2025::include_file;
use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
struct IngredientIdRange(RangeInclusive<usize>);

impl IngredientIdRange {
    // returns Some(range) if combine can occur, else None
    pub fn merge(&self, other: &IngredientIdRange) -> Result<IngredientIdRange, String> {
        if self.end() >= other.start() {
            Ok(IngredientIdRange(
                *self.start().min(other.start())..=*self.end().max(other.end()),
            ))
        } else {
            Err("Unable to merge".to_string())
        }
    }
}

impl FromStr for IngredientIdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((start_str, end_str)) = s.split_once("-") else {
            return Err(format!("Unable to parse range: {}", s).to_string());
        };
        let (start, end) = (
            start_str.parse::<usize>().expect(&format!(
                "Expected start of range but received: {}",
                start_str
            )),
            end_str.parse::<usize>().expect(&format!(
                "Expected end of range but received: {}",
                start_str
            )),
        );

        Ok(IngredientIdRange(start..=end))
    }
}

impl Deref for IngredientIdRange {
    type Target = RangeInclusive<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let input = include_file!("input");
    let Some((raw_ranges, raw_ids)) = input.split_once("\n\n") else {
        panic!("Failed to split ranges from ids");
    };

    let mut ranges = raw_ranges
        .lines()
        .map(IngredientIdRange::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Expected ranges to parse, but failed");

    let ingredient_ids = raw_ids
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    ranges.sort_by(|a, b| a.start().cmp(&b.start()));

    let mut merged_ranges = Vec::new();

    let mut curr_range = ranges[0].clone();
    for range in ranges.iter().skip(1) {
        if let Ok(merged) = curr_range.merge(&range) {
            curr_range = merged;
        } else {
            merged_ranges.push(curr_range);
            curr_range = range.clone();
        }
    }
    merged_ranges.push(curr_range);

    let mut num_fresh_ingrient_ids = 0;
    for id in ingredient_ids {
        for IngredientIdRange(range) in merged_ranges.iter() {
            if range.contains(&id) {
                num_fresh_ingrient_ids += 1;
                break;
            }
        }
    }

    let num_fresh_ingredient_ids_total = merged_ranges
        .iter()
        .map(|r| (r.end() - r.start()) + 1)
        .sum::<usize>();


    println!("Part 1: {}", num_fresh_ingrient_ids);
    println!("Part 2: {}", num_fresh_ingredient_ids_total);

}
