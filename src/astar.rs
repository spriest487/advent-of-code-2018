use {
    std::{
        collections::{
            HashSet,
            HashMap,
        },
        cmp::Ordering,
        ops::Add,
        hash::Hash,
    },
};

pub trait Heuristic {
    type Item: Clone + Eq + PartialEq + Hash;
    type Score: Clone + Add<Output=Self::Score> + PartialOrd;

    fn score(from: &Self::Item, to: &Self::Item) -> Self::Score;

    fn zero_score() -> Self::Score;
    fn infinity_score() -> Self::Score;

}

pub trait World {
    type Point:  Clone + Eq + PartialEq + Hash;
    type Score: Clone + Add<Output=Self::Score> + PartialOrd;

    type Neighbors: Iterator<Item=Self::Point>;
    type Heuristic: Heuristic<Item=Self::Point, Score=Self::Score>;

    fn neighbors(origin: &Self::Point) -> Self::Neighbors;
    fn neighbor_dist() -> Self::Score;

    fn point_order(a: &Self::Point, b: &Self::Point) -> Ordering;
}

pub struct Pathfinder<W: World> {
    open: HashSet<W::Point>,
    closed: HashSet<W::Point>,
    came_from: HashMap<W::Point, W::Point>,
    g_score: HashMap<W::Point, W::Score>,
    f_score: HashMap<W::Point, W::Score>,
}

impl<W: World> Pathfinder<W> {
    pub fn new() -> Self {
        Self {
            open: HashSet::new(),
            closed: HashSet::new(),
            came_from: HashMap::new(),
            g_score: HashMap::new(),
            f_score: HashMap::new(),
        }
    }

    fn score_or_infinity(scores: &HashMap<W::Point, W::Score>, point: &W::Point) -> W::Score {
        scores.get(point).cloned().unwrap_or(W::Heuristic::infinity_score())
    }

    pub fn find_path(&mut self,
                 origin: W::Point,
                 dest: W::Point,
                 tile_pred: impl Fn(&W::Point) -> bool,
                 out: &mut Vec<W::Point>) -> bool {
        self.came_from.clear();
        self.closed.clear();

        self.open.clear();
        self.open.insert(origin.clone());

        self.g_score.clear();
        self.g_score.insert(origin.clone(), W::Heuristic::zero_score());

        self.f_score.clear();
        self.f_score.insert(origin.clone(), W::Heuristic::score(&origin, &dest));

        loop {
            if self.open.is_empty() {
                break false;
            }

            let current = self.next();

            if current == dest {
                self.reconstruct_path(current, out);
                break true;
            }

            self.open.remove(&current);
            self.closed.insert(current.clone());

            let neighbors = W::neighbors(&current).filter(|p| tile_pred(p));

            for neighbor in neighbors {
                if self.closed.contains(&neighbor) {
                    // neighbor is already evaluated
                    continue;
                }

                // the distance from start to a neighbor
                let tentative_gscore = Self::score_or_infinity(&self.g_score, &current) + W::neighbor_dist();

                if !self.open.insert(neighbor.clone())
                    && tentative_gscore >= Self::score_or_infinity(&self.g_score, &neighbor) {
                    // this is not a better path
                    continue;
                }

                // this path is the best until now, record it
                self.came_from.insert(neighbor.clone(), current.clone());
                self.g_score.insert(neighbor.clone(), tentative_gscore.clone());
                self.f_score.insert(neighbor.clone(), tentative_gscore + W::Heuristic::score(&neighbor, &dest));
            }
        }
    }

    fn next(&self) -> W::Point {
        self.f_score.iter()
            .filter(|(point, _)| self.open.contains(*point))
            .min_by(|(a_pos, a_score), (b_pos, b_score)| {
                match a_score.partial_cmp(b_score) {
                    Some(Ordering::Equal) | None => W::point_order(*a_pos, *b_pos),
                    Some(order) => order,
                }
            })
            .map(|(point, _score)| point.clone())
            .unwrap()
    }

    fn reconstruct_path(&mut self, mut current: W::Point, out: &mut Vec<W::Point>) {
        out.clear();
        out.push(current.clone());

        while let Some(next) = self.came_from.remove(&current) {
            current = next;

            out.push(current.clone());
        }

        out.reverse();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::point::*;

    fn load_test_area(map: &str) -> Vec<Point> {
        let mut points = Vec::new();

        let non_whitespace_liens = map.lines()
            .filter(|line| line.chars().any(|c| !c.is_ascii_whitespace()));

        for (y, line) in non_whitespace_liens.enumerate() {
            for (x, map_char) in line.trim().chars().enumerate() {
                if map_char != '#' {
                    points.push(Point::new(x as isize, y as isize));
                }
            }
        }

        points
    }

    #[test]
    fn finds_one_step_path() {
        let area = load_test_area(r"
            #####
            #   #
            #####");

        let mut pathfinder: Pathfinder<CavernWorld> = Pathfinder::new();
        let tile_pred = |p: &Point| area.contains(p);

        let mut path = Vec::new();
        let found = pathfinder.find_path(Point::new(1, 1), Point::new(2, 1), tile_pred, &mut path);

        assert!(found);
        assert_eq!(2, path.len());
        assert_eq!(Point::new(1, 1), path[0]);
        assert_eq!(Point::new(2, 1), path[1]);
    }

    #[test]
    fn finds_corner_path() {
        let area = load_test_area(r"
            #####
            #   #
            # ###
            #####");

        let mut pathfinder: Pathfinder<CavernWorld> = Pathfinder::new();
        let tile_pred = |p: &Point| area.contains(p);

        let mut path = Vec::new();
        let found = pathfinder.find_path(Point::new(1, 2), Point::new(3, 1), tile_pred, &mut path);

        assert!(found);
        assert_eq!(4, path.len(), "path should have length 4, found path: {:?}", path);
        assert_eq!(Point::new(1, 2), path[0]);
        assert_eq!(Point::new(1, 1), path[1]);
        assert_eq!(Point::new(2, 1), path[2]);
        assert_eq!(Point::new(3, 1), path[3]);
    }
}