use {
    regex::Regex,
    std::collections::hash_map::{Entry, HashMap},
};

struct Claim {
    id: usize,

    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn read_line(line: &str, pattern: &Regex) -> Self {
        let claim_match = pattern.captures(line).unwrap();
        let id: usize = claim_match["id"].parse().unwrap();
        let x: usize = claim_match["x"].parse().unwrap();
        let y: usize = claim_match["y"].parse().unwrap();
        let width: usize = claim_match["w"].parse().unwrap();
        let height: usize = claim_match["h"].parse().unwrap();

        Self {
            id,
            x,
            y,
            width,
            height,
        }
    }

    fn right(&self) -> usize {
        self.x + self.width
    }

    fn bottom(&self) -> usize {
        self.y + self.height
    }

    fn claim_points<'a>(&'a self, cloth: &mut HashMap<(usize, usize), Vec<&'a Self>>) {
        let xs = self.x..self.right();

        let coords = xs.flat_map(|x| {
            let ys = self.y..self.bottom();
            ys.map(move |y| (x, y))
        });

        for (x, y) in coords {
            match cloth.entry((x, y)) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(self);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![self]);
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("day3.txt");
    let claim_pattern = Regex::new(
        r"(?x)
        \#(?P<id>[0-9]+)\s@\s
        (?P<x>[0-9]+),
        (?P<y>[0-9]+):\s
        (?P<w>[0-9]+)x
        (?P<h>[0-9]+)",
    )
    .unwrap();

    let claims: Vec<_> = input
        .lines()
        .map(|line| Claim::read_line(line, &claim_pattern))
        .collect();

    let mut cloth = HashMap::new();
    for claim in claims.iter() {
        claim.claim_points(&mut cloth);
    }

    let dup_claims = cloth.values().filter(|claims| claims.len() > 1).count();

    println!("number of duplicate claimed coordinates: {}", dup_claims);

    let winning_claim_id = claims
        .iter()
        .filter_map(|claim| {
            for x in claim.x..claim.right() {
                for y in claim.y..claim.bottom() {
                    if cloth[&(x, y)].len() > 1 {
                        return None;
                    }
                }
            }
            Some(claim.id)
        })
        .next()
        .unwrap();

    println!("winning claim: {}", winning_claim_id);
}
