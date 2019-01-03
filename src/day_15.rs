mod point;
mod astar;

use {
    crate::{
        astar::Pathfinder,
        point::{
            Point,
            Neighbors,
        },
    },
    std::{
        fmt,
        cmp::Ordering,
        collections::{
            HashSet,
            HashMap,
        },
        time::Instant,
        usize,
    },
    rayon::prelude::*,
};

pub struct ManhattanDistHeuristic;

impl astar::Heuristic for ManhattanDistHeuristic {
    type Item = Point;
    type Score = usize;

    fn score(from: &Point, to: &Point) -> usize { from.manhattan_dist_to(*to) }
    fn zero_score() -> usize { 0 }
    fn infinity_score() -> usize { usize::MAX }
}

pub struct CavernWorld;

impl astar::World for CavernWorld {
    type Point = Point;
    type Score = usize;
    type Neighbors = Neighbors;

    type Heuristic = ManhattanDistHeuristic;

    fn neighbors(origin: &Point) -> Neighbors { origin.neighbors_reading_order() }
    fn neighbor_dist() -> usize { 1 }
    fn point_order(a: &Point, b: &Point) -> Ordering { Point::cmp_reading_order(*a, *b) }
}

type CavernPathfinder = Pathfinder<CavernWorld>;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Team {
    Elf,
    Goblin,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Team::Goblin => "Goblin",
            Team::Elf => "Elf",
        })
    }
}

#[derive(Clone)]
struct Fighter {
    team: Team,
    pos: Point,

    hp: isize,
}

const BASE_ATTACK_POWER: isize = 3;

impl Fighter {
    fn new(team: Team, pos: Point) -> Self {
        Self {
            team,
            pos,
            hp: 200,
        }
    }
}

impl fmt::Debug for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fighter ( {} @ {} )", self.team, self.pos)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Blocked,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Blocked => write!(f, "#"),
        }
    }
}

#[derive(Clone)]
struct Cavern {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    fighters: Vec<Fighter>,
    fighter_positions: HashMap<Point, usize>,

    elf_attack_power: isize,
}

impl Cavern {
    fn parse(s: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut fighters = Vec::new();
        let mut tiles = Vec::new();

        for (y, line) in s.lines().enumerate() {
            height += 1;
            width = line.len(); // assume all lines are the same length

            for (x, char) in line.chars().enumerate() {
                let point = Point::new(x as isize, y as isize);
                match char {
                    '#' => tiles.push(Tile::Blocked),
                    'E' => {
                        tiles.push(Tile::Empty);
                        fighters.push(Fighter::new(Team::Elf, point));
                    }
                    'G' => {
                        tiles.push(Tile::Empty);
                        fighters.push(Fighter::new(Team::Goblin, point));
                    }
                    _ => tiles.push(Tile::Empty),
                }
            }
        }

        let mut cavern = Self {
            tiles,
            width,
            height,
            fighters,
            fighter_positions: HashMap::new(),

            elf_attack_power: BASE_ATTACK_POWER,
        };

        cavern.refresh_fighter_positions();
        cavern
    }

    fn refresh_fighter_positions(&mut self) {
        self.fighter_positions.clear();
        for (i, f) in self.fighters.iter().enumerate() {
            self.fighter_positions.insert(f.pos, i);
        }
    }

    fn is_free_space(&self, point: Point) -> bool {
        match self.tile_at(point) {
            Tile::Empty => self.fighter_at(point).is_none(),
            Tile::Blocked => false,
        }
    }

    fn fighter_at(&self, point: Point) -> Option<usize> {
        self.fighter_positions.get(&point)
            .filter(|&&i| self.fighters[i].hp > 0)
            .cloned()
    }

    fn tile_at(&self, point: Point) -> Tile {
        let off = self.width as isize * point.y + point.x;
        if off >= 0 && off < self.tiles.len() as isize {
            self.tiles[off as usize]
        } else {
            Tile::Blocked
        }
    }

    fn find_targets(&self, i: usize, targets: &mut Vec<usize>) {
        targets.clear();
        let fighter = &self.fighters[i];

        targets.extend(self.fighters.iter().enumerate()
            .filter(|(_, other)| other.hp > 0)
            .filter_map(|(j, other)| if other.team != fighter.team {
                Some(j)
            } else {
                None
            }));
    }

    fn move_fighter(&mut self, i: usize, targets: &[usize], pathfinder: &mut CavernPathfinder) {
        let fighter = &self.fighters[i];

        let dests: HashSet<_> = targets.iter()
            .flat_map(|j| {
                let target_pos = self.fighters[*j].pos;
                target_pos.neighbors_reading_order()
            })
            .filter(|p| self.is_free_space(*p) || *p == fighter.pos)
            .collect();

        if !dests.contains(&fighter.pos) {
            let mut paths = Vec::new();

            let origin_points = fighter.pos.neighbors_reading_order()
                .filter(|p| self.is_free_space(*p));

            let mut path = Vec::new();
            for origin in origin_points {
                for &dest in &dests {
                    let free_tile_pred = |p: &Point| self.is_free_space(*p);

                    if pathfinder.find_path(origin, dest, free_tile_pred, &mut path) {
                        paths.push(path.clone());
                        path.clear();
                    }
                }
            }

            paths.sort_by(|a, b| {
                let a_dest = *a.last().unwrap();
                let b_dest = *b.last().unwrap();
                // sort first by shortest paths...
                match a.len().cmp(&b.len()) {
                    // then by origin pos in reading order
                    Ordering::Equal => Point::cmp_reading_order(a_dest, b_dest),
                    dest_order => dest_order,
                }
            });

            if !paths.is_empty() {
                // move this fighter to the first step of the chosen path
                self.fighters[i].pos = paths[0][0];
                self.refresh_fighter_positions();
            }
        }
    }

    fn resolve_attacks(&mut self, i: usize) {
        let neighbors = self.fighters[i].pos.neighbors_reading_order();

        let target_index = neighbors
            .filter_map(|neighbor| {
                self.fighters.iter().enumerate()
                    .filter_map(|(j, f)| {
                        if f.pos == neighbor
                            && f.hp > 0
                            && f.team != self.fighters[i].team {
                            Some(j)
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .min_by(|a, b| {
                let a = &self.fighters[*a];
                let b = &self.fighters[*b];
                match a.hp.cmp(&b.hp) {
                    Ordering::Equal => Point::cmp_reading_order(a.pos, b.pos),
                    hp_order => hp_order,
                }
            });

        if let Some(j) = target_index {
            let attack_power = match self.fighters[i].team {
                Team::Elf => self.elf_attack_power,
                Team::Goblin => BASE_ATTACK_POWER,
            };

            self.fighters[j].hp = isize::max(0, self.fighters[j].hp - attack_power);
        }
    }

    fn tick(&mut self, pathfinder: &mut CavernPathfinder) -> Option<Team> {
        let mut targets = Vec::new();

        self.fighters.sort_by(|a, b| Point::cmp_reading_order(a.pos, b.pos));
        self.refresh_fighter_positions();

        for i in 0..self.fighters.len() {
            if self.fighters[i].hp > 0 {
                self.find_targets(i, &mut targets);
                if targets.is_empty() {
                    let winner = self.fighters[i].team;

                    // all enemies are dead, battle is over
                    return Some(winner);
                }

                self.move_fighter(i, &targets, pathfinder);
                self.resolve_attacks(i);
            }
        }

        None
    }

    fn elves(&self) -> impl Iterator<Item=&Fighter> {
        self.fighters.iter().filter(|f| f.hp > 0 && f.team == Team::Elf)
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                let pos = Point::new(x, y);
                match self.fighter_at(pos) {
                    Some(fighter_pos) => match self.fighters[fighter_pos].team {
                        Team::Elf => write!(f, "E")?,
                        Team::Goblin => write!(f, "G")?,
                    }
                    None => write!(f, "{}", self.tile_at(pos))?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Outcome {
    elf_power: isize,
    elves_remaining: Vec<Fighter>,
    winner: Team,
    hp_sum: isize,
    time: isize,
}

impl Outcome {
    fn new(cavern: &Cavern, winner: Team, time: isize) -> Self {
        let hp_sum = cavern.fighters.iter().map(|f| f.hp).sum::<isize>();


        Self {
            hp_sum,
            elf_power: cavern.elf_attack_power,
            elves_remaining: cavern.elves().cloned().collect(),
            winner,
            time,
        }
    }

    fn value(&self) -> isize {
        self.hp_sum * self.time
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t\tteam survived after {}\trounds * {}\t\tremaining HP = {},\telf power = {},\tsurviving elves = {}",
               self.winner,
               self.time,
               self.hp_sum,
               self.value(),
               self.elf_power,
               self.elves_remaining.len())
    }
}

fn main() {
    let input = include_str!("day_15.txt");
    let initial_state = Cavern::parse(input);

    let total_start_time = Instant::now();

    let initial_elves = initial_state.elves().count();

    let chunk_size: isize = 8;

    let mut winning_outcomes = (0..).filter_map(|chunk| {
        let chunk_outcomes: Vec<Outcome> = (0..chunk_size).into_par_iter()
            .map(|i| {
                let mut pathfinder = CavernPathfinder::new();
                let attack_boost = (chunk_size * chunk + i) as isize;

                let mut cavern = initial_state.clone();
                cavern.elf_attack_power += attack_boost;

                let mut time = 0;
                loop {
                    if let Some(winner) = cavern.tick(&mut pathfinder) {
                        break Outcome::new(&cavern, winner, time);
                    } else {
                        time += 1;
                    }
                }
            })
            .collect();

        chunk_outcomes.into_iter()
            .inspect(|outcome| println!("{}", outcome))
            .find(|outcome| outcome.elves_remaining.len() == initial_elves)
    });

    let winning_outcome = winning_outcomes.next().unwrap();

    println!("final outcome: {}", winning_outcome);

    for elf in &winning_outcome.elves_remaining {
        println!("  surviving elf with {} HP", elf.hp);
    }

    let total_elapsed = Instant::now() - total_start_time;
    println!("elapsed time: {}.{}s", total_elapsed.as_secs(), total_elapsed.subsec_millis());
}
