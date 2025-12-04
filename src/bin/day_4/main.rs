use std::collections::HashMap;
use advent_of_code_2025::{include_file, Vec2i};

#[derive(Eq, PartialOrd, PartialEq, Debug, Copy, Clone)]
enum Spot {
    Roll,
    Free,
}

fn main() {
    let input = include_file!("sample");
    let mut map = HashMap::<Vec2i, Spot>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, spot) in line.chars().enumerate() {
            map.insert(Vec2i::new(x as i64, y as i64), if spot == '@' { Spot::Roll } else { Spot::Free });
        }
    }

    let mut all_forkliftable_spots = Vec::new();
    let mut curr_spots_len = all_forkliftable_spots.len();
    let mut surrounding_spots = Vec::new();
    for y in -1..=1 {
        for x in -1..=1 {
            if x == 0 && y == 0 {
                continue
            }
            surrounding_spots.push(Vec2i::new(x, y));
        }
    }

    let mut part_1_spot_count = 0;
    loop {
        let mut forkliftable_spots = Vec::<(Vec2i, Spot)>::new();
        for (pos, spot) in map.iter() {
            if spot != &Spot::Roll {
                continue
            }
            let mut surrounding_roll_count = 0;
            for &Vec2i {x, y} in surrounding_spots.iter() {
                let check_pos = *pos + Vec2i::new(x, y);
                if let Some(Spot::Roll) = map.get(&check_pos) {
                    surrounding_roll_count += 1;
                }
            }
            if surrounding_roll_count < 4 {
                forkliftable_spots.push((*pos, *spot));
            }
        }
        for (pos, spot) in forkliftable_spots {
            map.remove(&pos);
            all_forkliftable_spots.push(spot);
        }
        if part_1_spot_count == 0 {
           part_1_spot_count = all_forkliftable_spots.len();
        }
        if all_forkliftable_spots.len() == curr_spots_len {
            break;
        }
        curr_spots_len = all_forkliftable_spots.len();
    }

    println!("Part 1: {}", part_1_spot_count);
    println!("Part 2: {}", all_forkliftable_spots.len());
}
