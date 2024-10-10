pub(crate) struct AnotherCoffee<'a, C, S> {
    name: &'a str,
    cost: C,
    size: S,
}
pub(crate) fn generics() {
    let coffee1 = AnotherCoffee {
        name: "c1",
        cost: 23,
        size: 30,
    };
    let coffee2 = AnotherCoffee {
        name: "c2",
        cost: 34.12,
        size: "grande",
    };
}
