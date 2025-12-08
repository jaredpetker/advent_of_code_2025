use advent_of_code_2025::{Vec3i, include_file};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct JunctionBoxConnection {
    first: Vec3i,
    second: Vec3i,
    distance: i64,
}

impl JunctionBoxConnection {
    fn from_junctions(first: Vec3i, second: Vec3i) -> JunctionBoxConnection {
        JunctionBoxConnection {
            first,
            second,
            distance: first.distance(&second),
        }
    }
}

impl Ord for JunctionBoxConnection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for JunctionBoxConnection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Circuit(HashSet<Vec3i>);

impl FromIterator<Vec3i> for Circuit {
    fn from_iter<T: IntoIterator<Item = Vec3i>>(iter: T) -> Self {
        Circuit(HashSet::from_iter(iter))
    }
}

impl Deref for Circuit {
    type Target = HashSet<Vec3i>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Circuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct CircuitGroup(Vec<Circuit>);

impl Deref for CircuitGroup {
    type Target = Vec<Circuit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CircuitGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CircuitGroup {
    fn merge_circuits_at(&mut self, circuit_idx_left: usize, circuit_idx_right: usize) {
        let (left, right) = self.0.split_at_mut(circuit_idx_right);
        left[circuit_idx_left].extend(right[0].iter());
        self.remove(circuit_idx_right);
    }

    fn add_junction_box_connection(&mut self, conn: &JunctionBoxConnection) {
        let circuits_with_junction_boxes = self._find_junction_boxes(&conn.first, &conn.second);

        if circuits_with_junction_boxes.len() == 0 {
            // junction boxes are in no existing circuits -- add a new circuit
            self.push(Circuit::from_iter([conn.first, conn.second]));
        } else if circuits_with_junction_boxes.len() == 1 {
            // a junction box is found in  one existing circuit -- add to it
            self[circuits_with_junction_boxes[0]].extend([conn.second, conn.first]);
        } else if circuits_with_junction_boxes.len() == 2 {
            // both junction boxes exist in two separate circuits -- merge the two circuits
            self.merge_circuits_at(
                circuits_with_junction_boxes[0],
                circuits_with_junction_boxes[1],
            );
        }
    }

    fn _find_junction_boxes(&self, first: &Vec3i, second: &Vec3i) -> Vec<usize> {
        // collect indices of circuits within the circuit group which contains each
        // junction box in the connection pair

        let mut found_circuits = Vec::new();
        for (circuit_idx, circuit) in self.iter().enumerate() {
            if circuit.contains(first) {
                found_circuits.push(circuit_idx);
            } else if circuit.contains(second) {
                found_circuits.push(circuit_idx);
            }

            if found_circuits.len() == 2 {
                break;
            }
        }
        found_circuits
    }
}

fn main() {
    let input = include_file!("input");

    let positions = input.lines().map(Vec3i::from).collect::<Vec<_>>();

    // pair up all the junction boxes
    let mut junction_pairs = Vec::new();
    for i in 0..positions.len() {
        let curr = &positions[i];
        for j in (i + 1)..positions.len() {
            let next = &positions[j];
            junction_pairs.push(JunctionBoxConnection::from_junctions(
                curr.clone(),
                next.clone(),
            ));
        }
    }
    junction_pairs.sort();

    let mut circuit_group = CircuitGroup(Vec::new());
    let mut positions_set = HashSet::<Vec3i>::from_iter(positions);
    let part_1_pairs_threshold = 1000;

    let mut part_1 = 0;
    let mut part_2 = 0;

    for (pair_idx, conn) in junction_pairs.iter().enumerate() {
        circuit_group.add_junction_box_connection(&conn);

        // remove junction boxes from the set of junction box positions
        positions_set.remove(&conn.first);
        positions_set.remove(&conn.second);

        if part_1_pairs_threshold == pair_idx + 1 {
            circuit_group.sort_by(|a, b| b.len().cmp(&a.len()));
            part_1 = circuit_group.iter().take(3).map(|c| c.len()).product();
        }

        // all junction boxes accounted for and are within a single circuit
        if positions_set.len() == 0 && circuit_group.len() == 1 {
            part_2 = conn.first.x * conn.second.x;
            break;
        }
    }

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}
