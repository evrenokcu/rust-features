use std::clone;

use crate::{print_header, print_sub_header};

pub(crate) fn execute() {
    print_header("extension traits");
    ext_trait_to_a_type();
    ext_trait_to_a_trait();
}

fn ext_trait_to_a_trait() {
    print_sub_header("apply extension trait to a trait");
    let v = vec![1, 2, 3];
    let r = v.iter().to_reversed();

    println!("vector: {:#?}", &v);
    println!("reversed vector: {:#?}", &r);
}

fn ext_trait_to_a_type() {
    print_sub_header("apply extension trait to a type");
    let v = vec![1, 2, 3];
    let r = &v.reversed();

    println!("vector: {:#?}", &v);
    println!("reversed vector: {:#?}", &r);
}

//'Ext' is a naming convension
pub trait ReverseExt<T> {
    fn reversed(&self) -> Vec<T>;
}
impl<T> ReverseExt<T> for Vec<T>
where
    T: Clone,
{
    fn reversed(&self) -> Vec<T> {
        self.iter().rev().cloned().collect()
    }
}

pub trait DoubleEndedITeratorExt: DoubleEndedIterator {
    fn to_reversed<'a, T>(self) -> Vec<T>
    where
        T: 'a + Clone,
        Self: Sized + Iterator<Item = &'a T>;
}

// to implement trait for a trait generic parameter I is introduced, check below
impl<I: DoubleEndedIterator> DoubleEndedITeratorExt for I {
    fn to_reversed<'a, T>(self) -> Vec<T>
    where
        T: 'a + Clone,
        Self: Sized + Iterator<Item = &'a T>,
    {
        self.rev().cloned().collect()
    }
}
