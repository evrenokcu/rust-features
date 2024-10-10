use std::cell::Cell;

use crate::print_sub_header;

pub(crate) fn execute() {
    print_sub_header("Cell<T>");
    let list = ExpensiveCalculator::new(&[1, 2, 3, 4, 5]);
    println!("Sum:{}", list.calculate_sum());
    println!("Sum:{}", list.calculate_sum());
    println!("Sum:{}", list.calculate_sum());
}

struct ExpensiveCalculator {
    data: Vec<i32>,
    sum: Cell<Option<i32>>,
}

impl ExpensiveCalculator {
    fn new(data: &[i32]) -> Self {
        Self {
            data: data.into(),
            sum: Cell::new(None),
        }
    }
    fn calculate_sum(&self) -> i32 {
        if self.sum.get().is_none() {
            let x: i32 = self.data.iter().sum();
            self.sum.set(Some(x));
            println!("calculating sum")
        }
        self.sum.get().unwrap_or_default()
    }
}
