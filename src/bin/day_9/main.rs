use advent_of_code_2025::{Vec2f, Vec2i, include_file};


fn main() {
    let input = include_file!("input");

    let corners = input
        .lines()
        .map(|line| line.split_once(',').expect("Expected x,y"))
        .map(Vec2i::from)
        .collect::<Vec<_>>();

    let mut edges = vec![];
    for i in 0..corners.len() {
        let j = if i == corners.len() - 1 { 0 } else { i + 1 };
        edges.push((i, j));
    }

    let mut largest_rect_area = 0;
    let mut largest_rect_area_in_poly = 0;
    for i in 0..corners.len() {
        'next_corner: for j in (i + 1)..corners.len() {
            let rect_corner_1 = corners[i];
            let rect_corner_2 = corners[j];

            let min_x = rect_corner_1.x.min(rect_corner_2.x);
            let min_y = rect_corner_1.y.min(rect_corner_2.y);

            let max_x = rect_corner_1.x.max(rect_corner_2.x);
            let max_y = rect_corner_1.y.max(rect_corner_2.y);

            let rect_area = (max_x - min_x + 1) * (max_y - min_y + 1);

            largest_rect_area = largest_rect_area.max(rect_area);

            let rect_center = Vec2f {
                x: ((min_x + max_x) as f64) / 2f64,
                y: ((min_y + max_y) as f64) / 2f64,
            };

            let ray_dir = Vec2f { x: 0f64, y: 1f64 };

            let mut num_rect_center_to_poly_intersections = 0;
            for &(corner_1_idx, corner_2_idx) in &edges {
                let edge_start = corners[corner_1_idx];
                let edge_end = corners[corner_2_idx];

                let is_corner_in_rect = edge_start.x > min_x
                    && edge_start.x < max_x
                    && edge_start.y > min_y
                    && edge_start.y < max_y;
                let rect_intersects_poly =
                    intersect(edge_start, edge_end, rect_corner_1, rect_corner_2).is_some();

                if is_corner_in_rect || rect_intersects_poly {
                    continue 'next_corner;
                }

                if line_ray_intersect(rect_center, ray_dir, edge_start.into(), edge_end.into())
                    .is_some()
                {
                    num_rect_center_to_poly_intersections += 1;
                }
            }

            // center is inside rect if it passes through poly an odd number of times
            if num_rect_center_to_poly_intersections % 2 == 0 {
                continue;
            }

            largest_rect_area_in_poly = largest_rect_area_in_poly.max(rect_area);
        }
    }

    println!("{}", largest_rect_area);
    println!("{}", largest_rect_area_in_poly);
}

fn intersect(a_start: Vec2i, a_end: Vec2i, b_start: Vec2i, b_end: Vec2i) -> Option<Vec2f> {
    let a_start = Vec2f::from(a_start);
    let a_end = Vec2f::from(a_end);
    let b_start = Vec2f::from(b_start);
    let b_end = Vec2f::from(b_end);
    intersect_f(a_start, a_end, b_start, b_end)
}

// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
fn intersect_f(a_start: Vec2f, a_end: Vec2f, b_start: Vec2f, b_end: Vec2f) -> Option<Vec2f> {
    let denominator = (b_end.y - b_start.y) * (a_end.x - a_start.x) - (b_end.x - b_start.x) * (a_end.y - a_start.y);

    if denominator == 0f64 {
        return None;
    }

    let t = ((b_end.x - b_start.x) * (a_start.y - b_start.y) - (b_end.y - b_start.y) * (a_start.x - b_start.x)) / denominator;
    let u = ((a_end.x - a_start.x) * (a_start.y - b_start.y) - (a_end.y - a_start.y) * (a_start.x - b_start.x)) / denominator;

    // this is a slight modification here, as  we need to ignore the ends vs >=, <=, etc.
    if t > 0f64 && t < 1f64 && u > 0f64 && u < 1f64 {
        let intersection = Vec2f {
            x: a_start.x + t * (a_end.x - a_start.x),
            y: a_start.y + t * (a_end.y - a_start.y),
        };
        Some(intersection)
    } else {
        None
    }
}

// https://rootllama.wordpress.com/2014/06/20/ray-line-segment-intersection-test-in-2d/
fn line_ray_intersect(
    ray_origin: Vec2f,
    ray_dir: Vec2f,
    edge_start: Vec2f,
    edge_end: Vec2f,
) -> Option<Vec2f> {
    let v1: Vec2f = Vec2f {
        x: ray_origin.x - edge_start.x,
        y: ray_origin.y - edge_start.y,
    };

    let v2: Vec2f = Vec2f {
        x: edge_end.x - edge_start.x,
        y: edge_end.y - edge_start.y,
    };

    let v3: Vec2f = Vec2f {
        x: -ray_dir.y,
        y: ray_dir.x,
    };

    let determinant = v2.dot(&v3);

    // potentially bad, should likely be checking is under some epsilon
    if determinant.abs() == 0f64 {
        return None;
    }

    // ray interp
    let t1 = v2.cross(&v1) / determinant;
    // line interp
    let t2 = v1.dot(&v3) / determinant;

    if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
        Some(Vec2f {
            x: ray_origin.x + t1 * ray_dir.x,
            y: ray_origin.y + t1 * ray_dir.y,
        })
    } else {
        None
    }
}
