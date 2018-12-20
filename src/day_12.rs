use std::collections::{HashMap, VecDeque};

use self::Pot::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Pot {
    NoPlant,
    Plant,
}

impl From<char> for Pot {
    fn from(c: char) -> Self {
        match c {
            '#' => Plant,
            '.' => NoPlant,
            _ => panic!("invalid pot {}", c),
        }
    }
}

type Rules = HashMap<[Pot; 5], Pot>;

#[derive(Debug)]
struct Input {
    initial_state: Vec<Pot>,
    rules: Rules,
}

impl Input {
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let initial_state_str = lines.next().unwrap();
        let initial_state: Vec<_> = initial_state_str["initial state: ".len()..]
            .chars()
            .map(Pot::from)
            .collect();

        lines.next().expect("empty line");

        let rules = lines.map(|line| {
            let mut pots = [NoPlant; 5];
            for (i, pot_char) in line[0..5].chars().enumerate() {
                pots[i] = Pot::from(pot_char);
            }

            let produces_char = line[5 + " => ".len()..].chars().next().unwrap();
            let produces = Pot::from(produces_char);

            (pots, produces)
        });

        Self {
            initial_state,
            rules: rules.collect(),
        }
    }
}

#[derive(Clone)]
struct Generation {
    pots: VecDeque<Pot>,
    zero_index: isize,
}

impl Generation {
    fn pots_at(&self, index: isize) -> [Pot; 5] {
        let mut pots = [NoPlant; 5];
        for i in 0..5 {
            let pot_index = index + (i - 2);
            pots[i as usize] = if pot_index < 0 || pot_index >= self.pots.len() as isize {
                NoPlant
            } else {
                self.pots[pot_index as usize]
            };
        }
        pots
    }

    fn next(&self, rules: &Rules) -> Self {
        let mut new_gen = self.clone();

        for i in 0..self.pots.len() {
            new_gen.pots[i] = rules[&self.pots_at(i as isize)];
        }

        /* we have to look up to two pots to the left and right */
        for left in 1..=2 {
            if rules[&self.pots_at(-left)] == Plant {
                new_gen.pots.push_front(Plant);
                new_gen.zero_index += 1;
            }
        }
        for right in 1..=2 {
            let right_index = (self.pots.len() - 1) + right;
            if rules[&self.pots_at(right_index as isize)] == Plant {
                new_gen.pots.push_back(Plant);
            }
        }

        new_gen
    }

    fn plant_labels(&self) -> impl Iterator<Item=isize> + '_ {
        self.pots.iter().enumerate()
            .filter_map(move |(i, pot)| match pot {
                Plant => Some(i as isize - self.zero_index),
                NoPlant => None,
            })
    }
}

const GENERATIONS: usize = 20;

fn main() {
    let Input {
        rules,
        initial_state,
    } = Input::parse(include_str!("day_12.txt"));

    let initial = Generation {
        pots: initial_state.into_iter().collect(),
        zero_index: 0,
    };

    let result = (0..GENERATIONS).fold(initial.clone(), |last_gen, _gen| {
        let next_gen = last_gen.next(&rules);
        next_gen
    });

    println!(
        "number of plants at gen {}: {} (label sum: {})",
        GENERATIONS,
        result.plant_labels().count(),
        result.plant_labels().sum::<isize>()
    );

    // iterate until we find a stable sum
    let (stable_gen, stable_sum, stable_dist) = {
        let mut next_gen = initial;

        let mut last_sum = next_gen.plant_labels().sum::<isize>();
        let mut last_sum_dist = 0;
        let mut gen = 1;

        loop {
            next_gen = next_gen.next(&rules);
            let sum = next_gen.plant_labels().sum::<isize>();
            let sum_dist = sum - last_sum;

            if sum_dist == last_sum_dist {
                break (gen, sum, sum_dist);
            } else {
                last_sum_dist = sum_dist;
                last_sum = sum;
                gen += 1;
            }
        }
    };

    println!("stable sum dist at gen {}: {}", stable_gen, stable_dist);
    let rest_gens = 50_000_000_000 - stable_gen;
    println!("sum at gen 50bn: {}", stable_sum + rest_gens * stable_dist);
}
