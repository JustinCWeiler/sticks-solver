use super::player::*;

// represents an action a player can take
pub enum Action {
    // attacking hand, victim idx, victim hand
    ATTACK(Hand, usize, Hand),
    // same as Player::split
    SPLIT(Hand, usize),
}

// represents current game state
#[derive(Debug, Clone)]
pub struct State<const COUNT: usize, const MAX: usize> {
    players: [Player<MAX>; COUNT],
    turn: usize,
}

// player 0 starts
impl<const COUNT: usize, const MAX: usize> Default for State<COUNT, MAX> {
    fn default() -> Self {
        State {
            players: [Player::default(); COUNT],
            turn: 0,
        }
    }
}

impl<const COUNT: usize, const MAX: usize> State<COUNT, MAX> {
    pub fn action(mut self, action: Action) -> State<COUNT, MAX> {
        match action {
            Action::ATTACK(attacking_hand, other_player_idx, other_hand) => {
                if other_player_idx != self.turn && other_player_idx < COUNT {
                    let player = self.players[self.turn];
                    let other_player = self.players[other_player_idx];

                    if let Some(new_other_player) =
                        player.attack(attacking_hand, other_player, other_hand)
                    {
                        self.players[other_player_idx] = new_other_player;

                        self.turn = (self.turn + 1) % COUNT;
                    }
                }
            }
            Action::SPLIT(hand, amount) => {
                let player = self.players[self.turn];

                if let Some(new_player) = player.split(hand, amount) {
                    self.players[self.turn] = new_player;

                    self.turn = (self.turn + 1) % COUNT;
                }
            }
        }

        self
    }
}
