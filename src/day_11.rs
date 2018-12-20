use {
    rayon::prelude::*,
};

const SERIAL: isize = 9445;
const GRID_SIZE: usize = 300;

fn power_level(x: isize, y: isize, serial_number: isize) -> isize {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial_number;
    power *= rack_id;
    power = (power % 1000) / 100;
    power -= 5;
    power
}

struct Result {
    x: usize,
    y: usize,
    size: usize,
    power_level: isize,
}

fn index_at(x: usize, y: usize) -> usize {
    y * GRID_SIZE + x
}

fn find_max(grid: &[isize], size: usize) -> Result {
    let mut max = None;

    for y in 0..GRID_SIZE - (size - 1) {
        for x in 0..GRID_SIZE - (size - 1) {
            let mut sum = 0;
            for local_x in 0..size {
                for local_y in 0..size {
                    sum += grid[index_at(x + local_x, y + local_y)];
                }
            }

            let result = (x, y, sum);

            max = match max {
                None => Some(result),
                Some((_, _, old_max)) if old_max < sum => Some(result),
                _ => max,
            };
        }
    }

    let (x, y, power_level) = max.expect(&format!("must have a max level for size {}", size));
    Result { x, y, power_level, size }
}

fn main() {
    assert_eq!(4, power_level(3, 5, 8), "power at 3, 5 with serial number 8");
    assert_eq!(-5, power_level(122, 79, 57), "power at 122, 79 with serial number 57");
    assert_eq!(0, power_level(217, 196, 39), "power at 217, 196 with serial number 39", );
    assert_eq!(4, power_level(101, 153, 71), "power at 101, 153 with serial number 71");

    let mut grid = [0; GRID_SIZE * GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            grid[index_at(x, y)] = power_level(x as isize, y as isize, SERIAL);
        }
    }

    let result = (0..300usize).into_par_iter()
        .map(|size| find_max(&grid, size + 1))
        .max_by_key(|result| result.power_level)
        .unwrap();

    println!("highest power level is at {}, {} size {} with level {}",
        result.x, result.y, result.size, result.power_level);
}