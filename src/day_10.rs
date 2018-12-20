use {
    std::{
        fmt,
        mem,
    },
    regex::Regex,
};

#[derive(Eq, PartialEq, Clone)]
struct Vector2 {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Entry {
    position: Vector2,
    velocity: Vector2,
}

impl Entry {
    fn parse(s: &str, entry_pattern: &Regex) -> Self {
        let groups = entry_pattern.captures(s).unwrap();
        //format: "position=<xx, xx> velocity=<xx, xx>"
        let pos_x: i32 = groups[1].parse().unwrap();
        let pos_y: i32 = groups[2].parse().unwrap();
        let vel_x: i32 = groups[3].parse().unwrap();
        let vel_y: i32 = groups[4].parse().unwrap();

        Self {
            position: Vector2 { x: pos_x, y: pos_y },
            velocity: Vector2 { x: vel_x, y: vel_y },
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position<{:2}, {:2}> velocity<{:2}, {:2}>",
               self.position.x, self.position.y,
               self.velocity.x, self.velocity.y)
    }
}

fn next_state(entries: &[Entry], next: &mut [Entry]) {
    for (i, entry) in entries.iter().enumerate() {
        next[i].position.x = entry.position.x + entry.velocity.x;
        next[i].position.y = entry.position.y + entry.velocity.y;
    }
}

fn bounds(entries: &[Entry]) -> (Vector2, Vector2) {
    let min_x = entries.iter().fold(0, |min, entry| i32::min(entry.position.x, min));
    let max_x = entries.iter().fold(0, |max, entry| i32::max(entry.position.x, max));
    let min_y = entries.iter().fold(0, |min, entry| i32::min(entry.position.y, min));
    let max_y = entries.iter().fold(0, |max, entry| i32::max(entry.position.y, max));

    (Vector2 { x: min_x, y: min_y }, Vector2 { x: max_x, y: max_y })
}

fn bounds_size(entries: &[Entry]) -> i32 {
    let (min, max) = bounds(entries);
    let x_dist = (min.x - max.x).abs();
    let y_dist = (min.y - max.y).abs();
    x_dist + y_dist
}

fn main() {
    let entry_pattern = Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$")
        .unwrap();
    let input = include_str!("day_10.txt");

    let mut entries: Vec<_> = input.lines()
        .map(|line| Entry::parse(line, &entry_pattern))
        .collect();
    let mut next = entries.clone();

    let mut time = 0;

    loop {
        next_state(&entries, &mut next);
        if bounds_size(&next) > bounds_size(&entries) {
            print_scene(&entries);
            println!("time: {}", time);
            break;
        } else {
            mem::swap(&mut entries, &mut next);
            time += 1;
        }
    }
}

fn print_scene(entries: &[Entry]) {
    let (min, max) = bounds(entries);

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let entry = entries.iter()
                .find(|entry| entry.position == Vector2 { x, y });

            print!("{}", match entry {
                Some(_) => '#',
                None => '.'
            });
        }
        println!();
    }
    println!();
}