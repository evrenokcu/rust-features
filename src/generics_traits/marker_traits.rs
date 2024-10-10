use std::{fmt::Debug, hash::Hash};

use crate::print_header;

pub(crate) fn execute() {
    print_header("marker traits");
    let c = FullFeaturedContainer { item: MyStruct {} };
    println!("{:?}", &c)
}

trait FullFeatured {}
impl<T> FullFeatured for T where
    T: Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd
{
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct MyStruct;

#[derive(Debug)]
struct FullFeaturedContainer<T: FullFeatured> {
    item: T,
}
