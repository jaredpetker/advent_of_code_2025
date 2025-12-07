use advent_of_code_2025::include_file;
use std::collections::{BTreeSet, HashSet};

enum Op {
    Sum,
    Product,
}

enum Line {
    Op(Op),
    Number,
}

impl From<char> for Line {
    fn from(value: char) -> Self {
        match value {
            '*' => Self::Op(Op::Product),
            '+' => Self::Op(Op::Sum),
            _ => Self::Number,
        }
    }
}

fn main() {
    let input = include_file!("input");

    // when we see an empty space, add position to our set
    // remove position when we see a non-empty space
    // do not re-add removed positions
    let mut columns_set = BTreeSet::new();
    let mut removed_set = HashSet::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c.is_whitespace() && !removed_set.contains(&i) {
                columns_set.insert(i);
            } else {
                columns_set.remove(&i);
                removed_set.insert(i);
            }
        }
    }
    let columns: Vec<usize> = columns_set.into_iter().collect();

    let mut cursor = 0;
    let mut part_1_sum = 0;
    let mut part_2_sum = 0;
    for i in 0..=columns.len() {
        let column = columns.get(i);
        let mut items = Vec::new();
        for line in input.lines() {
            let end = column.cloned().unwrap_or(line.len());

            let section = &line[cursor..end];
            if let Line::Op(op) = Line::from(section.chars().next().expect("Expected a character"))
            {
                let max_digits = end - cursor;

                // create vertically read digits
                let numbers_vert = items
                    .iter()
                    .map(|p: &String| {
                        p.trim()
                            .parse::<usize>()
                            .expect(&format!("Expected parsable usize, but received: {}", p))
                    })
                    .collect::<Vec<usize>>();

                // create right-to-left read digits
                let mut digits = vec![Vec::<char>::new(); max_digits];
                for item in items.iter() {
                    for (i, c) in item.chars().enumerate() {
                        if !c.is_whitespace() {
                            digits[i].push(c);
                        }
                    }
                }
                let numbers_rtl = digits
                    .iter()
                    .map(|v| v.iter().collect::<String>())
                    .map(|s| s.parse::<usize>().expect("Expected numerical item"))
                    .collect::<Vec<usize>>();

                match op {
                    Op::Sum => {
                        part_1_sum += numbers_vert.iter().sum::<usize>();
                        part_2_sum += numbers_rtl.iter().sum::<usize>()
                    }
                    Op::Product => {
                        part_1_sum += numbers_vert.iter().product::<usize>();
                        part_2_sum += numbers_rtl.iter().product::<usize>()
                    }
                }

                // set next cursor position
                cursor = end + 1;
            } else {
                items.push(section.to_string());
            }
        }
    }

    println!("part 1: {}", part_1_sum);
    println!("part 2: {}", part_2_sum);
}
