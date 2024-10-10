trait Observer {
    fn update(&self);
}
trait Subject<'a> {
    fn attach(&mut self, observer: impl Observer + 'a);
    fn notify(&self);
}
struct FileSubject<'a> {
    observers: Vec<Box<dyn Observer + 'a>>,
}
impl<'a> Subject<'a> for FileSubject<'a> {
    fn attach(&mut self, observer: impl Observer + 'a) {
        self.observers.push(Box::new(observer));
    }

    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.update();
        }
    }
}

struct ObserverProcess {}

impl Observer for ObserverProcess {
    fn update(&self) {
        println!("First Observer Notified!");
    }
}

pub(crate) fn observer() {
    let mut s = FileSubject {
        observers: Vec::new(),
    };
    let o = ObserverProcess {};
    s.attach(o);
    s.notify();
}
