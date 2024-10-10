use std::sync::{Arc, Mutex};

use crate::print_header;

pub(crate) fn execute() {
    thread_local! {
        static CATS:Arc<Mutex<Option<Vec<String>>>>=Arc::new(Mutex::new(None));
    }
    print_header("global state");

    let arc = CATS.with(|arc| arc.clone());

    let mut inner = arc.lock().expect("UNABLE TO LOCK");
    *inner = Some(vec![String::from("Zarife"), String::from("Ruhi")]);
}
