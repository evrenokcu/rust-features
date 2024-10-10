use sample1::Greet;

use crate::{print_header, print_sub_header};

mod sample1 {
    pub(super) trait Speak {
        fn speak(words: &str) {
            println!("{}", words);
        }
    }

    pub(super) trait Greet: Speak {
        fn greet() {
            Self::speak("Hi Evren");
        }
    }

    pub(super) struct Robot {}

    impl Greet for Robot {}
    impl Speak for Robot {}
}

pub(crate) fn super_trait() {
    print_header("super traits");
    print_sub_header("version 1");
    sample1::Robot::greet();
    sample2::execute();
}

mod sample2 {
    use std::fmt::Debug;

    use crate::print_sub_header;

    pub(super) trait CloneAndDebug: Clone + Debug {
        fn clone_and_debug(&self) -> Self {
            let r = self.clone();
            dbg!(&r);
            r
        }
    }
    pub(super) fn execute() {
        print_sub_header("version 2");
        let s = MyStruct {};
        s.clone_and_debug();
    }
    #[derive(Clone, Debug)]
    struct MyStruct {}
    impl CloneAndDebug for MyStruct {}
}
