use advent_of_code_2025::include_file;

trait RepeatsN {
    fn repeats_n(&self, at_least: usize, at_most: usize) -> bool;
}

impl RepeatsN for String {
    fn repeats_n(&self, at_least: usize, at_most: usize) -> bool {
        'outer: for n_repeats in at_least..=at_most {
            if self.len() % n_repeats != 0 {
                continue;
            }
            let chunk_size = self.len() / n_repeats;
            let chars: Vec<char> = self.chars().collect();
            for segment_start in (chunk_size..self.len()).step_by(chunk_size) {
                for segment_idx in 0..chunk_size {
                    if chars[segment_start + segment_idx] != chars[segment_idx] {
                        continue 'outer;
                    }
                }
            }
            return true
        }
        false
    }
}


fn main() {
    let input = include_file!("input");
    let ranges = input
        .split(',')
        .map(|range| range.split('-'))
        .map(|mut pair| {
            let start = pair.next()
                .expect("Expected start for range")
                .to_string()
                .parse::<usize>()
                .expect("Expected number");
            let end = pair.next()
                .expect("Expected end for range")
                .to_string()
                .parse::<usize>()
                .expect("Expected number");
            start..=end
        })
        .collect::<Vec<_>>();

    let mut part_1_sum = 0;
    let mut part_2_sum = 0;

    for range in ranges {
        for product_id in range {
            let product_id_str = product_id.to_string();
            if product_id_str.repeats_n(2, 2) {
                part_1_sum += product_id;
            }
            if product_id_str.repeats_n(2, product_id_str.len()) {
                part_2_sum += product_id;
            }
        }
    }

    println!("Part 1: {:?}", part_1_sum);
    println!("Part 2: {:?}", part_2_sum);
}
