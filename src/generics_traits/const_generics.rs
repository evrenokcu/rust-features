use crate::print_header;

pub(crate) fn execute() {
    print_header("constant generics");
    let buf = MyBuffer::from([0, 2, 4, 5]);
    dbg!(&buf);
}

#[derive(Debug)]
struct MyBuffer<T, const LENGTH: usize> {
    buf: [T; LENGTH],
}

impl<T, const LENGTH: usize> From<[T; LENGTH]> for MyBuffer<T, LENGTH> {
    fn from(value: [T; LENGTH]) -> Self {
        MyBuffer { buf: value }
    }
}
