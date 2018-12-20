use {
    std::{
        fmt,
        collections::HashMap,
        cmp::Ordering,
        ops::Add,
    },
};

#[derive(Debug, Copy, Clone)]
enum CartFacing {
    North,
    South,
    East,
    West,
}

impl CartFacing {
    fn parse(c: char) -> Option<CartFacing> {
        match c {
            '^' => Some(CartFacing::North),
            'v' => Some(CartFacing::South),
            '<' => Some(CartFacing::West),
            '>' => Some(CartFacing::East),
            _ => None,
        }
    }

    fn underlying_section(&self) -> Section {
        match self {
            CartFacing::North | CartFacing::South => Section::Vertical,
            CartFacing::East | CartFacing::West => Section::Horizontal,
        }
    }

    fn velocity(&self) -> Coord {
        match self {
            CartFacing::North => Coord { x: 0, y: -1 },
            CartFacing::South => Coord { x: 0, y: 1 },
            CartFacing::East => Coord { x: 1, y: 0 },
            CartFacing::West => Coord { x: -1, y: 0 },
        }
    }

    fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Left => match self {
                CartFacing::North => CartFacing::West,
                CartFacing::West => CartFacing::South,
                CartFacing::South => CartFacing::East,
                CartFacing::East => CartFacing::North,
            },
            Turn::Right => match self {
                CartFacing::North => CartFacing::East,
                CartFacing::East => CartFacing::South,
                CartFacing::South => CartFacing::West,
                CartFacing::West => CartFacing::North,
            },
            Turn::Straight => self.clone(),
        }
    }
}

impl fmt::Display for CartFacing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            CartFacing::North => '^',
            CartFacing::South => 'v',
            CartFacing::East => '>',
            CartFacing::West => '<',
        })
    }
}

#[derive(Debug)]
enum Curve {
    Right,
    Left
}

impl Curve {
    fn turn(&self, from_facing: CartFacing) -> Turn {
        match self {
            Curve::Right => match from_facing {
                CartFacing::North | CartFacing::South => Turn::Right,
                CartFacing::East | CartFacing::West => Turn::Left,
            }
            Curve::Left => match from_facing {
                CartFacing::East | CartFacing::West => Turn::Right,
                CartFacing::North | CartFacing::South => Turn::Left,
            }
        }
    }
}

#[derive(Debug)]
enum Section {
    Vertical,
    Horizontal,
    Intersection,
    Curve(Curve),
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Section::Vertical => '|',
            Section::Horizontal => '-',
            Section::Intersection => '+',
            Section::Curve(Curve::Right) => '/',
            Section::Curve(Curve::Left) => '\\',
        })
    }
}

impl Section {
    fn parse(c: char) -> Option<Section> {
        match c {
            '|' => Some(Section::Vertical),
            '-' => Some(Section::Horizontal),
            '+' => Some(Section::Intersection),
            '/' => Some(Section::Curve(Curve::Right)),
            '\\' => Some(Section::Curve(Curve::Left)),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[derive(Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

struct Cart {
    pos: Coord,
    facing: CartFacing,
    last_turn: Turn,
}

impl Cart {
    fn new(pos: Coord, facing: CartFacing) -> Self {
        Self {
            pos,
            facing,
            last_turn: Turn::Right,
        }
    }

    fn intersection_turn(&mut self) -> Turn {
        let turn = match self.last_turn {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };
        self.last_turn = turn;
        turn
    }
}

struct Track {
    sections: HashMap<Coord, Section>,
    carts: Vec<Cart>,
    first_crash: Option<Coord>,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_x = self.sections.keys().map(|pos| pos.x).min().unwrap();
        let min_y = self.sections.keys().map(|pos| pos.y).min().unwrap();
        let max_x = self.sections.keys().map(|pos| pos.x).max().unwrap();
        let max_y = self.sections.keys().map(|pos| pos.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Coord { x, y };
                match self.carts.iter().find(|cart| cart.pos == pos) {
                    Some(cart) => write!(f, "{}", cart.facing)?,
                    None => match self.sections.get(&Coord { x, y }) {
                        Some(section) => write!(f, "{}", section)?,
                        None => write!(f, " ")?,
                    }
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

enum MoveResult {
    Ok,
    // Crash(other cart index)
    Crash(usize),
}

impl Track {
    fn parse(s: &str) -> Self {
        let tiles: Vec<_> = s.lines().enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let cart_facing = CartFacing::parse(c);
                    let section = cart_facing.as_ref()
                        .map(|c| c.underlying_section())
                        .or_else(|| Section::parse(c));
                    let pos = Coord { x: x as isize, y: y as isize };

                    section.map(|section| (pos, section, cart_facing))
                })
            })
            .collect();

        let carts = tiles.iter()
            .filter_map(|(pos, _, facing)| {
                facing.map(|facing| Cart::new(*pos, facing))
            })
            .collect();

        let sections = tiles.into_iter()
            .map(|(pos, section, _)| (pos, section))
            .collect();

        Self {
            sections,
            carts,
            first_crash: None,
        }
    }

    fn move_cart(&mut self, i: usize) -> MoveResult {
        self.carts[i].pos = self.carts[i].pos + self.carts[i].facing.velocity();

        for j in (0..self.carts.len()).filter(|j| *j != i) {
            if self.carts[i].pos == self.carts[j].pos {
                return MoveResult::Crash(j);
            }
        }

        let mut cart = &mut self.carts[i];
        let section = self.sections.get(&cart.pos)
            .unwrap_or_else(|| panic!("missing section at {},{}", cart.pos.x, cart.pos.y));

        let turn = match section {
            Section::Curve(curve) => curve.turn(cart.facing),
            Section::Intersection => cart.intersection_turn(),
            Section::Vertical | Section::Horizontal => Turn::Straight,
        };
        cart.facing = cart.facing.turn(turn);

        MoveResult::Ok
    }

    fn tick(&mut self) {
        self.carts.sort_by(|cart_a, cart_b| {
            match cart_a.pos.y.cmp(&cart_b.pos.y) {
                Ordering::Equal => cart_a.pos.x.cmp(&cart_b.pos.x),
                y_ord => y_ord,
            }
        });

        let mut i = 0;
        while i < self.carts.len() {
            match self.move_cart(i) {
                MoveResult::Ok => i += 1,
                MoveResult::Crash(j) => {
                    if self.first_crash.is_none() {
                        self.first_crash = Some(self.carts[i].pos);
                    }

                    if j < i {
                        self.carts.remove(j);
                        i -= 1;

                        self.carts.remove(i);
                    } else {
                        self.carts.remove(i);
                        self.carts.remove(j - 1);
                    }
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("day_13.txt");
    let mut track = Track::parse(input);

    for _time in 0.. {
        track.tick();

        if track.carts.len() == 0 {
            println!("all carts crashed");
            break;
        }

        if track.carts.len() == 1 {
            let last_pos = track.carts[0].pos;
            println!("surviving cart at {},{}", last_pos.x, last_pos.y);
            break;
        }
    }

    let first_crash = track.first_crash.unwrap();
    println!("first crash at {},{}", first_crash.x, first_crash.y);
}