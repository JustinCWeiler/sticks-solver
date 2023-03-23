mod state;
use state::*;

fn main() {
    let mut s: State<3, 5> = Default::default();

    println!("{:?}", s);

    s = s.action(Action::ATTACK(Hand::LEFT, 2, Hand::LEFT));

    println!("{:?}", s);

    s = s.action(Action::SPLIT(Hand::LEFT, 1));

    println!("{:?}", s);
}
