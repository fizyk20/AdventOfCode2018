const N_PLAYERS: usize = 477;

use std::collections::HashMap;

struct Marble {
    prev_marble: usize,
    next_marble: usize,
}

struct GameState {
    current_marble: usize,
    marbles: HashMap<usize, Marble>,
    points: [usize; N_PLAYERS],
    current_player: usize,
}

impl GameState {
    fn new(max: usize) -> Self {
        let mut marbles = HashMap::with_capacity(max + 1);
        let _ = marbles.insert(
            0,
            Marble {
                prev_marble: 0,
                next_marble: 0,
            },
        );
        Self {
            current_marble: 0,
            marbles,
            points: [0; N_PLAYERS],
            current_player: 0,
        }
    }

    fn insert_marble_after_current(&mut self, marble: usize) {
        let next_marble = self.marbles[&self.current_marble].next_marble;
        self.marbles
            .get_mut(&self.current_marble)
            .unwrap()
            .next_marble = marble;
        self.marbles.get_mut(&next_marble).unwrap().prev_marble = marble;
        self.marbles.insert(
            marble,
            Marble {
                prev_marble: self.current_marble,
                next_marble,
            },
        );
        self.current_marble = marble;
    }

    fn remove_current_marble(&mut self) {
        let prev_marble = self.marbles[&self.current_marble].prev_marble;
        let next_marble = self.marbles[&self.current_marble].next_marble;
        self.marbles.get_mut(&prev_marble).unwrap().next_marble = next_marble;
        self.marbles.get_mut(&next_marble).unwrap().prev_marble = prev_marble;
        let _ = self.marbles.remove(&self.current_marble);
        self.current_marble = next_marble;
    }

    fn move_clockwise(&mut self, amount: usize) {
        for _ in 0..amount {
            self.current_marble = self.marbles[&self.current_marble].next_marble;
        }
    }

    fn move_counterclockwise(&mut self, amount: usize) {
        for _ in 0..amount {
            self.current_marble = self.marbles[&self.current_marble].prev_marble;
        }
    }

    fn place_marble(&mut self, marble: usize) {
        if marble % 23 == 0 {
            self.points[self.current_player] += marble;
            self.move_counterclockwise(7);
            self.points[self.current_player] += self.current_marble;
            self.remove_current_marble();
        } else {
            self.move_clockwise(1);
            self.insert_marble_after_current(marble);
        }
        self.current_player = (self.current_player + 1) % N_PLAYERS;
    }

    fn max_score(&self) -> usize {
        *self.points.iter().max().unwrap()
    }
}

fn game_with_max_marble(max: usize) -> usize {
    let mut state = GameState::new(max);

    for marble in 1..=max {
        state.place_marble(marble);
    }

    state.max_score()
}

fn main() {
    println!("Part 1: {}", game_with_max_marble(70851));
    println!("Part 2: {}", game_with_max_marble(7085100));
}
