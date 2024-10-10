trait Observer {
    fn update(&self);
}

trait Subject<'a, T: Observer + 'a> {
    fn attach(&mut self, observer: &'a T); // Accepts a reference to observer
    fn notify(&self);
}

struct FileSubject<'a, T: Observer + 'a> {
    observers: Vec<&'a T>, // Stores references to observers
}

impl<'a, T: Observer + 'a> Subject<'a, T> for FileSubject<'a, T> {
    fn attach(&mut self, observer: &'a T) {
        self.observers.push(observer); // Store the reference in the Vec
    }

    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.update(); // Call update on each observer
        }
    }
}

// Concrete observer implementation
struct ObserverProcess;

impl Observer for ObserverProcess {
    fn update(&self) {
        println!("First Observer Notified!");
    }
}

// Test the functionality
pub(crate) fn observer() {
    let obs1 = ObserverProcess {}; // Define an observer
    let obs2 = ObserverProcess {}; // Another observer

    // Create a FileSubject with references to observers
    let mut s = FileSubject {
        observers: Vec::new(),
    };

    // Attach observers by reference
    s.attach(&obs1);
    s.attach(&obs2);

    // Notify all observers
    s.notify();
}
