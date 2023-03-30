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
struct StateD<const COUNT: usize, const MAX: usize> {
    players: [Player<MAX>; COUNT],
    turn: usize,
}

// player 0 starts
impl<const COUNT: usize, const MAX: usize> Default for StateD<COUNT, MAX> {
    fn default() -> Self {
        StateD {
            players: [Player::default(); COUNT],
            turn: 0,
        }
    }
}

impl<const COUNT: usize, const MAX: usize> StateD<COUNT, MAX> {
    pub fn action(mut self, action: Action) -> StateD<COUNT, MAX> {
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

#[derive(Clone, Copy)]
pub struct State<const COUNT: usize, const MAX: usize> {
    pub val: usize,
}

impl<const COUNT: usize, const MAX: usize> std::fmt::Debug for State<COUNT, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let s = self.deconstruct();
        s.fmt(f)
    }
}

impl<const COUNT: usize, const MAX: usize> Default for State<COUNT, MAX> {
    fn default() -> Self {
        let mut val = 0;

        for _ in 0..COUNT {
            val *= Self::PLAYER_MAX_VAL;
            val += MAX + 1;
        }

        State { val }
    }
}

impl<const COUNT: usize, const MAX: usize> State<COUNT, MAX> {
    const PLAYER_MAX_VAL: usize = MAX * (MAX + 1) / 2;
    const TOTAL_PLAYER_MAX_VAL: usize = Self::PLAYER_MAX_VAL.pow(COUNT as u32);
    pub const MAX_STATE_VAL: usize = Self::TOTAL_PLAYER_MAX_VAL * COUNT;

    fn construct(state: StateD<COUNT, MAX>) -> State<COUNT, MAX> {
        let mut val = 0;

        for i in 0..COUNT {
            val *= Self::PLAYER_MAX_VAL;
            val += state.players[i].val;
        }

        val += Self::TOTAL_PLAYER_MAX_VAL * state.turn;

        State { val }
    }

    fn deconstruct(self) -> StateD<COUNT, MAX> {
        let mut val = self.val;

        let mut new_state = StateD::default();

        new_state.turn = val / Self::TOTAL_PLAYER_MAX_VAL;
        val %= Self::TOTAL_PLAYER_MAX_VAL;

        let mut radix = Self::TOTAL_PLAYER_MAX_VAL;
        for i in 0..COUNT {
            radix /= Self::PLAYER_MAX_VAL;
            new_state.players[i].val = val / radix;
            val %= radix;
        }

        new_state
    }

    pub fn action(self, action: Action) -> State<COUNT, MAX> {
        let state_deconst = self.deconstruct();

        let new_state_deconst = state_deconst.action(action);

        Self::construct(new_state_deconst)
    }
}
