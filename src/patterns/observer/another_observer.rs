use std::{sync::Arc, sync::Weak};

use crate::print_header;

pub(crate) fn execute() {
    print_header("another observer");
    let observer_1 = MyObserver::new("observer 1");
    let observer_2 = MyObserver::new("observer 2");

    let mut subject = Subject::new("initial state");
    subject.attach(observer_1.clone());
    subject.attach(observer_2.clone());

    subject.update();
}
trait Observer {
    type Subject;

    fn observe(&self, subject: &Self::Subject);
}
trait Observable {
    type Observer;
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
    fn update(&self);
}
struct Subject {
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    state: String,
}
impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: Vec::new(),
            state: state.into(),
        }
    }
    pub fn state(&self) -> &str {
        &self.state
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }

    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.observe(self));
    }
}

struct MyObserver {
    name: String,
}

impl MyObserver {
    fn new(name: &str) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
        })
    }
}

impl Observer for MyObserver {
    type Subject = Subject;

    fn observe(&self, subject: &Self::Subject) {
        println!(
            "observed from observer:{}, subject state{}",
            self.name, subject.state
        )
    }
}
