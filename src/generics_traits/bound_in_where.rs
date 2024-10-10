struct Wrapper<T> {
    item: T,
}
impl<T> Wrapper<T>
where
    //pay attention below, T should be converted to String
    String: From<T>,
    T: Clone,
{
    fn print(&self) {
        let x = &self.item;
        let s: String = String::from(x.to_owned());
        println!("{s}");
    }
}

pub(crate) fn bound_in_where() {
    crate::print_header("bound in where");

    let t = Wrapper {
        item: MyType {
            my_type_string: "Evren".to_string(),
        },
    };
    t.print();
}
#[derive(Clone)]
struct MyType {
    my_type_string: String,
}
impl From<MyType> for String {
    fn from(value: MyType) -> Self {
        value.my_type_string
    }
}
