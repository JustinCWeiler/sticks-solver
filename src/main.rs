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
}
