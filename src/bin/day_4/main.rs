use advent_of_code_2025::{include_file, Vec2i};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Roll(Vec2i);

fn main() {
    let input = include_file!("sample");
    let mut rolls = HashSet::<Roll>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, spot) in line.chars().enumerate() {
            if spot == '@' {
                rolls.insert(Roll(Vec2i::new(x as i64, y as i64)));
            }
        }
    }

    let mut all_forkliftable_rolls = Vec::new();
    let mut curr_rolls_len = all_forkliftable_rolls.len();
    let mut surrounding_spots = Vec::new();
    for y in -1..=1 {
        for x in -1..=1 {
            if x == 0 && y == 0 {
                continue
            }
            surrounding_spots.push(Vec2i::new(x, y));
        }
    }

    let mut part_1_roll_count = 0;
    loop {
        let mut forkliftable_rolls = Vec::<Roll>::new();
        for Roll(pos) in rolls.iter() {
            let mut surrounding_roll_count = 0;
            for &Vec2i {x, y} in surrounding_spots.iter() {
                let check_pos = *pos + Vec2i::new(x, y);
                if let Some(_) = rolls.get(&Roll(check_pos)) {
                    surrounding_roll_count += 1;
                }
            }
            if surrounding_roll_count < 4 {
                forkliftable_rolls.push(Roll(*pos));
            }
        }
        for roll in forkliftable_rolls {
            rolls.remove(&roll);
            all_forkliftable_rolls.push(roll);
        }
        if part_1_roll_count == 0 {
           part_1_roll_count = all_forkliftable_rolls.len();
        }
        if all_forkliftable_rolls.len() == curr_rolls_len {
            break;
        }
        curr_rolls_len = all_forkliftable_rolls.len();
    }

    println!("Part 1: {}", part_1_roll_count);
    println!("Part 2: {}", all_forkliftable_rolls.len());
}
