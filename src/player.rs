#[derive(PartialEq, Eq)]
pub enum Hand {
    LEFT,
    RIGHT,
}

// Player Deconstructed
#[derive(Debug)]
struct PlayerD<const MAX: usize> {
    left: usize,
    right: usize,
}

impl<const MAX: usize> PlayerD<MAX> {
    fn attack(
        self,
        attacking_hand: Hand,
        mut other_player: PlayerD<MAX>,
        other_hand: Hand,
    ) -> Option<PlayerD<MAX>> {
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

    fn split(mut self, hand: Hand, amount: usize) -> Option<PlayerD<MAX>> {
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
        p.fmt(f)
    }
}

impl<const MAX: usize> Default for Player<MAX> {
    fn default() -> Self {
        Player { val: MAX + 1 }
    }
}

impl<const MAX: usize> Player<MAX> {
    fn deconstruct(self) -> PlayerD<MAX> {
        PlayerD {
            left: self.val / MAX,
            right: self.val % MAX,
        }
    }

    fn construct(opt_player: Option<PlayerD<MAX>>) -> Option<Player<MAX>> {
        if let Some(mut player) = opt_player {
            // normalize
            player.left %= MAX;
            player.right %= MAX;
            if player.right < player.left {
                let tmp = player.left;
                player.left = player.right;
                player.right = tmp;
            }

            Some(Player {
                val: player.left * MAX + player.right,
            })
        } else {
            None
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

        let new_pd = pd.attack(attacking_hand, other_pd, other_hand);
        Self::construct(new_pd)
    }

    pub fn split(self, hand: Hand, amount: usize) -> Option<Player<MAX>> {
        let pd = self.deconstruct();

        let new_pd = pd.split(hand, amount);
        Self::construct(new_pd)
    }
}
