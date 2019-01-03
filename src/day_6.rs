mod point;

use {
    std::{
        collections::hash_map::HashMap,
    },
    crate::point::{Point, BoundingBox},
};

fn parse_coord(s: &str) -> Point {
    let comma = s.find(',').unwrap();
    let (x_str, y_str) = s.split_at(comma);

    Point {
        x: x_str.parse().unwrap(),
        y: y_str[2..].parse().unwrap(),
    }
}

fn main() {
    let input = include_str!("day_6.txt");
    let coords: Vec<_> = input.lines().map(parse_coord).collect();

    let bounds = BoundingBox::of_points(coords.iter().cloned());

    let mut closest_coords_count = HashMap::new();

    for location in bounds.coords() {
        let mut distances: Vec<_> = coords.iter()
            .enumerate()
            .map(|(i, coord)| (i, location.manhattan_dist_to(*coord)))
            .collect();

        distances.sort_unstable_by_key(|(_i, dist)| *dist);

        let (closest_coord, closest_dist) = distances[0];
        let (_next_coord, next_dist) = distances[1];

        if closest_dist != next_dist {
            closest_coords_count.entry(closest_coord)
                .or_insert_with(Vec::new)
                .push(location);
        }
    }

    let infinite_coords: Vec<_> = closest_coords_count.iter()
        .filter_map(|(coord, locs)| if locs.iter()
            .any(|loc| bounds.on_edge(*loc)) {
            Some(*coord)
        } else {
            None
        })
        .collect();
    for infinite_coord in infinite_coords {
        closest_coords_count.remove(&infinite_coord);
    }

    let largest = closest_coords_count.iter()
        .max_by_key(|(_coord, locs)| locs.len())
        .unwrap();

    println!("coord with largest area: #{} (count: {})", largest.0, largest.1.len());

    const SAFE_DIST: usize = 10000;

    let safe_region_size = bounds.coords()
        .filter_map(|location| {
            let dist_to_all: usize = coords.iter()
                .map(|coord| coord.manhattan_dist_to(location))
                .sum();

            if dist_to_all < SAFE_DIST {
                Some(location)
            } else {
                None
            }
        })
        .count();

    println!("safe region where all coords are within {} size: {}", SAFE_DIST, safe_region_size);
}
