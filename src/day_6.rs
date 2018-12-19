use {
    std::{
        collections::hash_map::HashMap,
    },
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn parse(s: &str) -> Self {
        let comma = s.find(',').unwrap();
        let (x_str, y_str) = s.split_at(comma);

        Self {
            x: x_str.parse().unwrap(),
            y: y_str[2..].parse().unwrap(),
        }
    }

    fn bounding_box<'a>(coords: impl IntoIterator<Item=&'a Self>) -> BoundingBox {
        let mut coords = coords.into_iter();

        let (mut min, mut max) = {
            let first = coords.next().unwrap();
            (first.clone(), first.clone())
        };

        while let Some(coord) = coords.next() {
            min.x = i32::min(coord.x, min.x);
            min.y = i32::min(coord.y, min.y);
            max.x = i32::max(coord.x, max.x);
            max.y = i32::max(coord.y, max.y);
        }

        BoundingBox { min, max }
    }

    fn manhattan_distance(&self, to: Coord) -> i32 {
        let x_diff = (self.x - to.x).abs();
        let y_diff = (self.y - to.y).abs();

        x_diff + y_diff
    }
}

#[derive(Debug)]
struct BoundingBox {
    min: Coord,
    max: Coord,
}

impl BoundingBox {
    fn on_edge(&self, coord: Coord) -> bool {
        coord.x == self.min.x
            || coord.y == self.min.y
            || coord.x == self.max.x
            || coord.y == self.max.y
    }

    fn coords(&self) -> impl Iterator<Item=Coord> + '_ {
        (self.min.x..=self.max.x)
            .flat_map(move |x| {
                (self.min.y..=self.max.y).map(move |y| {
                    Coord { x, y }
                })
            })
    }
}

fn main() {
    let input = include_str!("day_6.txt");
    let coords: Vec<_> = input.lines().map(Coord::parse).collect();

    let bounds = Coord::bounding_box(coords.iter());

    let mut closest_coords_count = HashMap::new();

    for location in bounds.coords() {
        let mut distances: Vec<_> = coords.iter()
            .enumerate()
            .map(|(i, coord)| (i, location.manhattan_distance(*coord)))
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

    const SAFE_DIST: i32 = 10000;

    let safe_region_size = bounds.coords()
        .filter_map(|location| {
            let dist_to_all: i32 = coords.iter()
                .map(|coord| coord.manhattan_distance(location))
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
