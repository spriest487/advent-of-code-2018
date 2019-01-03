use {
    std::{
        ops::Add,
        cmp::Ordering,
        fmt,
        usize,
    },
};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[allow(unused)]
impl Point {
    const NORTH: Point = Point { x: 0, y: -1 };
    const SOUTH: Point = Point { x: 0, y: 1 };
    const EAST: Point = Point { x: 1, y: 0 };
    const WEST: Point = Point { x: -1, y: 0 };

    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub fn north(self) -> Self {
        self + Self::NORTH
    }

    pub fn south(self) -> Self {
        self + Self::SOUTH
    }

    pub fn east(self) -> Self {
        self + Self::EAST
    }

    pub fn west(self) -> Self {
        self + Self::WEST
    }

    pub fn neighbors_reading_order(self) -> Neighbors {
        Neighbors { point: self, dir: 0 }
    }

    pub fn cmp_reading_order(self, other: Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            order => order,
        }
    }

    pub fn dist_to(self, other: Self) -> f32 {
        let dist_x = (self.x - other.x).abs();
        let dist_y = (self.y - other.y).abs();
        f32::sqrt((dist_x * dist_x) as f32 + (dist_y * dist_y) as f32)
    }

    pub fn manhattan_dist_to(self, other: Self) -> usize {
        let dist_x = (self.x - other.x).abs();
        let dist_y = (self.y - other.y).abs();

        dist_x as usize + dist_y as usize
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Neighbors {
    point: Point,
    dir: usize,
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let next = match self.dir {
            0 => Some(self.point.north()),
            1 => Some(self.point.west()),
            2 => Some(self.point.east()),
            3 => Some(self.point.south()),
            _ => None,
        };
        self.dir += 1;
        next
    }
}