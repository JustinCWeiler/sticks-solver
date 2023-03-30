mod player;
mod state;

use player::*;
use state::*;

fn main() {
    let mut s: State<3, 5> = Default::default();

    println!("{:?}", s);

    s = s.action(Action::ATTACK(Hand::LEFT, 2, Hand::LEFT));

    println!("{:?}", s);

    s = s.action(Action::SPLIT(Hand::RIGHT, 1));

    println!("{:?}", s);

    for val in 0..Player::<5>::MAX_PLAYER_VAL {
        println!("{:?}", Player::<5> { val });
    }

    for val in 0..State::<3, 5>::MAX_STATE_VAL {
        println!("{:?}", State::<3, 5> { val });
    }
}
