use {
    std::{
        fmt,
    },
};

const PLAYERS: usize = 418;
const HIGHEST_MARBLE: usize = 7076900;

struct Marble {
    value: usize,
}

struct Board {
    storage: Vec<Position>,
    head: usize,
}

struct Position {
    ccw: usize,
    cw: usize,
    marble: Marble,
}

enum PlayResult {
    Placed,
    Scored(usize),
}

impl Board {
    fn new() -> Self {
        let mut storage = Vec::new();
        storage.push(Position {
            ccw: 0,
            cw: 0,
            marble: Marble { value: 0 }
        });

        Self {
            storage,
            head: 0,
        }
    }

    fn cw_index(&mut self, dist: usize) -> usize {
        (0..dist).fold(self.head, |next, _| {
            self.storage[next].cw
        })
    }

    fn ccw_index(&mut self, dist: usize) -> usize {
        (0..dist).fold(self.head, |next, _| {
            self.storage[next].ccw
        })
    }

    fn play(&mut self, marble: Marble) -> PlayResult {
        if marble.value % 23 == 0 {
            let remove_at = self.ccw_index(7);

            let removed_val = self.storage[remove_at].marble.value;

            let removed_parent = self.storage[remove_at].ccw;
            self.storage[removed_parent].cw = self.storage[remove_at].cw;
            self.head = self.storage[removed_parent].cw;

            PlayResult::Scored(removed_val + marble.value)
        } else {
            let insert_after = self.cw_index(1);
            let inserted_before = self.cw_index(2);

            let new_pos = Position {
                ccw: insert_after,
                cw: self.storage[insert_after].cw,
                marble,
            };

            self.storage.push(new_pos);
            self.head = self.storage.len() - 1;

            self.storage[inserted_before].ccw = self.head;
            self.storage[insert_after].cw = self.head;

            PlayResult::Placed
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Board [")?;

        let mut next = self.head;
        loop {
            let marble = self.storage[next].marble.value;

            if next == self.head {
                write!(f, "({:3})",  marble)?;
            } else {
                write!(f, " {:3} ", marble)?;
            }

            next = self.storage[next].cw;
            if next == self.head {
                break;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    let mut scores = [0; PLAYERS];
    let mut marbles = (1..=HIGHEST_MARBLE)
        .map(|value| Marble { value })
        .into_iter();

    let mut board = Board::new();

    'game: loop {
        for player in 0..PLAYERS {
            let next = match marbles.next() {
                Some(marble) => marble,
                None => break 'game,
            };

            match board.play(next) {
                PlayResult::Placed => {}
                PlayResult::Scored(score) => {
                    scores[player] += score;
                }
            }
        }
    }

    let (winner, win_score) = scores.iter().enumerate()
        .max_by_key(|(_, score)| **score)
        .map(|(i, _)| (i, scores[i]))
        .unwrap();

    println!("winner is player {} with score {}", winner + 1, win_score);
}