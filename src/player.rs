#[derive(PartialEq, Eq)]
pub enum Hand {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct PlayerDeconstructed<const MAX: usize> {
    left: usize,
    right: usize,
}

impl<const MAX: usize> PlayerDeconstructed<MAX> {
    fn attack(
        self,
        attacking_hand: Hand,
        mut other_player: PlayerDeconstructed<MAX>,
        other_hand: Hand,
    ) -> Option<PlayerDeconstructed<MAX>> {
        let amount = match attacking_hand {
            Hand::LEFT => self.left,
            Hand::RIGHT => self.right,
        };

        match other_hand {
            Hand::LEFT => {
                if other_player.left == 0 {
                    return None;
                }

                other_player.left += amount;
            }
            Hand::RIGHT => {
                if other_player.right == 0 {
                    return None;
                }

                other_player.right += amount;
            }
        }

        Some(other_player)
    }

    fn split(mut self, hand: Hand, amount: usize) -> Option<PlayerDeconstructed<MAX>> {
        match hand {
            Hand::LEFT => {
                self.left -= amount;

                if self.left == self.right {
                    return None;
                }

                self.right += amount;
            }
            Hand::RIGHT => {
                self.left += amount;

                if self.left == self.right {
                    return None;
                }

                self.right -= amount;
            }
        }

        Some(self)
    }
}

#[derive(Clone, Copy)]
pub struct Player<const MAX: usize> {
    pub val: usize,
}

impl<const MAX: usize> std::fmt::Debug for Player<MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let p = self.deconstruct();
        write!(f, "Player: {{ left: {}, right: {} }}", p.left, p.right)
    }
}

impl<const MAX: usize> Default for Player<MAX> {
    fn default() -> Self {
        Player { val: MAX + 1 }
    }
}

impl<const MAX: usize> Player<MAX> {
    fn deconstruct(self) -> PlayerDeconstructed<MAX> {
        PlayerDeconstructed {
            left: self.val / MAX,
            right: self.val % MAX,
        }
    }

    fn construct(mut player: PlayerDeconstructed<MAX>) -> Player<MAX> {
        // normalize
        player.left %= MAX;
        player.right %= MAX;
        if player.right < player.left {
            let tmp = player.left;
            player.left = player.right;
            player.right = tmp;
        }

        Player {
            val: player.left * MAX + player.right,
        }
    }

    pub fn attack(
        self,
        attacking_hand: Hand,
        other_player: Player<MAX>,
        other_hand: Hand,
    ) -> Option<Player<MAX>> {
        let pd = self.deconstruct();
        let other_pd = other_player.deconstruct();

        if let Some(new_pd) = pd.attack(attacking_hand, other_pd, other_hand) {
            Some(Self::construct(new_pd))
        } else {
            None
        }
    }

    pub fn split(self, hand: Hand, amount: usize) -> Option<Player<MAX>> {
        let pd = self.deconstruct();

        if let Some(new_pd) = pd.split(hand, amount) {
            Some(Self::construct(new_pd))
        } else {
            None
        }
    }
}
