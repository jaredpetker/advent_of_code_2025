use advent_of_code_2025::{Vec2i, include_file};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct ManifoldPos(Vec2i);

impl Ord for ManifoldPos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.y.cmp(&other.0.y).then(self.0.x.cmp(&other.0.x))
    }
}

impl PartialOrd for ManifoldPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Location {
    Start,
    Empty,
    Splitter,
}

impl From<char> for Location {
    fn from(value: char) -> Self {
        match value {
            'S' => Location::Start,
            '.' => Location::Empty,
            '^' => Location::Splitter,
            _ => panic!("Unknown char {}", value),
        }
    }
}

fn main() {
    let input = include_file!("input");

    let mut map = HashMap::<Vec2i, Location>::new();
    let mut start = Vec2i::new(0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let loc = Location::from(ch);
            if loc == Location::Start {
                start = Vec2i::new(x as i64, y as i64);
            }
            map.insert(Vec2i::new(x as i64, y as i64), Location::from(ch));
        }
    }

    // solution variables
    // part 1
    let mut num_splits = 0;
    // part2
    let mut total_timelines = 0;

    // map of positions to timeline counts
    // ManifoldPos imposes a different ordering than Vec2i, y position takes precedence
    let mut beams = BTreeMap::<ManifoldPos, usize>::from([(ManifoldPos(start), 1)]);

    while let Some(entry) = beams.pop_first() {
        let (ManifoldPos(curr_pos), num_timelines) = entry;
        let next_pos = curr_pos + Vec2i::down();
        if let Some(location) = map.get(&next_pos) {
            let positions = if let Location::Splitter = location {
                num_splits += 1;
                let left = next_pos + Vec2i::left();
                let right = next_pos + Vec2i::right();
                vec![left, right]
            } else {
                vec![next_pos]
            };
            // combine timelines on collisions
            for position in positions {
                beams
                    .entry(ManifoldPos(position))
                    .and_modify(|curr_timelines| *curr_timelines += num_timelines)
                    .or_insert(num_timelines);
            }
        } else {
            total_timelines += num_timelines;
        }
    }

    println!("part 1: {}", num_splits);
    println!("part 2: {}", total_timelines);
}
