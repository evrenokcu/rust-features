use std::marker::PhantomData;

use crate::print_header;

pub(crate) fn execute() {
    print_header("struct tagging");
    let bulb = LightBulb::<Off>::default();
    println!("bulb:{}", &bulb.state());
    let bulb = bulb.turn_on();
    println!("bulb:{}", &bulb.state());
    let bulb = bulb.turn_off();
    println!("bulb:{}", &bulb.state());
    let bulb = bulb.turn_on();
    println!("bulb:{}", &bulb.state());
}
#[derive(Default)]
struct LightBulb<State: BulbState> {
    phantom: PhantomData<State>,
}

#[derive(Default)]
struct On;
#[derive(Default)]
struct Off;

trait BulbState {}
impl BulbState for On {}
impl BulbState for Off {}

impl LightBulb<On> {
    fn turn_off(self) -> LightBulb<Off> {
        LightBulb::<Off>::default()
    }
    fn state(&self) -> &str {
        "On"
    }
}
impl LightBulb<Off> {
    fn turn_on(self) -> LightBulb<On> {
        LightBulb::<On>::default()
    }
    fn state(&self) -> &str {
        "Off"
    }
}
