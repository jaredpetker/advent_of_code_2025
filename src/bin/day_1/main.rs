use advent_of_code_2025::include_file;
use std::str::FromStr;

// Enum + more for rotational instructions
// Ended up being a bit of superfluous ceremony
enum RotInstr {
    Left(i32),
    Right(i32),
}

impl From<RotInstr> for i32 {
    fn from(value: RotInstr) -> Self {
        match value {
            RotInstr::Left(rot) => -rot,
            RotInstr::Right(rot) => rot,
        }
    }
}

// Trying to not use any external crates this year (like regex). Parsing by hand.
impl FromStr for RotInstr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, rot) = (
            s.chars()
                .next()
                .expect("Expected character but received nothing"),
            s[1..]
                .parse::<i32>()
                .expect(format!("Expected an i32 but received: {}", s[1..].to_string()).as_str()),
        );
        match dir {
            'L' => Ok(RotInstr::Left(rot)),
            'R' => Ok(RotInstr::Right(rot)),
            _ => Err(format!("Unknown rotation instruction: {}", s)),
        }
    }
}

fn main() {
    // let input = include_file!("sample");
    let input = include_file!("input");

    let instructions = input
        .lines()
        .map(|line| {
            RotInstr::from_str(line)
                .expect(format!("Expected a Rotation Instruction but received: {}", line).as_str())
        })
        .collect::<Vec<_>>();

    // initial dial position
    let mut dial_pos = 50;

    // part 1 result
    let mut num_zeroes_at_rest = 0;

    // part 2 result
    let mut num_zeroes_clicked_total = 0;

    for rot_instr in instructions {
        let instr: i32 = rot_instr.into();
        let unbounded_dial_pos = dial_pos + instr;
        let pre_rot_dial_pos = dial_pos;
        dial_pos = unbounded_dial_pos.rem_euclid(100).abs();

        // A bit of a silly way to go vs conditionals, but amusing
        num_zeroes_clicked_total += unbounded_dial_pos.abs() / 100
            // If we have rotated left past zero
            // and we weren't already at zero
            // then account for passing zero
            + ((1 - (pre_rot_dial_pos == 0) as i32) * (unbounded_dial_pos <= 0) as i32);
        
        num_zeroes_at_rest += (dial_pos == 0) as i32;
    }

    println!("Part 1: {}", num_zeroes_at_rest);
    println!("Part 2: {}", num_zeroes_clicked_total);
}
