use std::f32::consts::PI;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

use crate::print_sub_header;
pub(crate) fn execute() {
    print_sub_header("coroutine sample 1");
    let mut yield_pi = #[coroutine]
    || {
        yield PI;
        "Coroutine completed!"
    };

    loop {
        match Pin::new(&mut yield_pi).resume(()) {
            CoroutineState::Yielded(val) => {
                dbg!(&val);
            }
            CoroutineState::Complete(val) => {
                dbg!(&val);
                break;
            }
        }
    }
}
