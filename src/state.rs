#[derive(PartialEq)]
pub enum Hand {
    LEFT,
    RIGHT,
}

// represents a player
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player<const MAX: usize> {
    left: usize,
    right: usize,
}

impl<const MAX: usize> Default for Player<MAX> {
    // player starts with 1 for each hand
    fn default() -> Self {
        Player { left: 1, right: 1 }
    }
}

// takes a player and returns that player but with the smaller number in the left hand
fn normalize_player<const MAX: usize>(mut player: Player<MAX>) -> Player<MAX> {
    player.left %= MAX;
    player.right %= MAX;
    if player.right < player.left {
        let tmp = player.left;
        player.left = player.right;
        player.right = tmp;
    }
    player
}

impl<const MAX: usize> Player<MAX> {
    // returns a new player that is the result of attacking other_player's other_hand with my_hand
    pub fn attack(
        &self,
        attacking_hand: Hand,
        mut other_player: Player<MAX>,
        other_hand: Hand,
    ) -> Player<MAX> {
        let amount = match attacking_hand {
            Hand::LEFT => self.left,
            Hand::RIGHT => self.right,
        };

        match other_hand {
            Hand::LEFT => {
                other_player.left += amount;
            }
            Hand::RIGHT => {
                other_player.right += amount;
            }
        }

        normalize_player(other_player)
    }

    // returns a new player that is the result of taking amount from hand and giving it to the
    // other hand
    pub fn split(mut self, hand: Hand, amount: usize) -> Player<MAX> {
        match hand {
            Hand::LEFT => {
                self.left -= amount;
                self.right += amount;
            }
            Hand::RIGHT => {
                self.left += amount;
                self.right -= amount;
            }
        }

        normalize_player(self)
    }
}

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
                    let other_player = self.players[other_player_idx];
                    self.players[other_player_idx] =
                        self.players[self.turn].attack(attacking_hand, other_player, other_hand);

                    self.turn = (self.turn + 1) % COUNT;
                }
            }
            Action::SPLIT(hand, amount) => {
                let mut player = self.players[self.turn];
                if (hand == Hand::LEFT
                    && player.left >= amount
                    && player.left - amount != player.right)
                    || (hand == Hand::RIGHT
                        && player.right >= amount
                        && player.right - amount != player.left)
                {
                    player = player.split(hand, amount);

                    self.players[self.turn] = player;

                    self.turn = (self.turn + 1) % COUNT;
                }
            }
        }

        self
    }
}
