const N_PLAYERS: usize = 477;
const MAX_MARBLE: usize = 70851;

struct GameState {
    current_marble_index: usize,
    marbles: Vec<usize>,
    points: [usize; N_PLAYERS],
    current_player: usize,
}

impl GameState {
    fn new() -> Self {
        Self {
            current_marble_index: 0,
            marbles: vec![0],
            points: [0; N_PLAYERS],
            current_player: 0,
        }
    }

    fn place_marble(&mut self, marble: usize) {
        if marble % 23 == 0 {
            self.points[self.current_player] += marble;
            let to_remove = if self.current_marble_index >= 7 {
                self.current_marble_index - 7
            } else {
                self.current_marble_index + self.marbles.len() - 7
            };
            let removed = self.marbles.remove(to_remove);
            self.current_marble_index = to_remove % self.marbles.len();
            self.points[self.current_player] += removed;
        } else {
            let to_insert = (self.current_marble_index + 2) % self.marbles.len();
            self.marbles.insert(to_insert, marble);
            self.current_marble_index = to_insert;
        }
        self.current_player = (self.current_player + 1) % N_PLAYERS;
    }

    fn max_score(&self) -> usize {
        *self.points.iter().max().unwrap()
    }
}

fn main() {
    let mut state = GameState::new();

    for marble in 1..=MAX_MARBLE {
        state.place_marble(marble);
    }

    println!("Part 1: {}", state.max_score());
}
