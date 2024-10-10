mod another_observer;
mod observer_with_generics;
mod observer_with_trait;
pub(crate) fn execute() {
    observer_with_trait::observer();
    observer_with_generics::observer();
    another_observer::execute();
}
