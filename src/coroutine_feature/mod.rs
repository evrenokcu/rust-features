use crate::print_header;
mod coroutine1;
mod coroutine_iterator;
pub(crate) fn execute() {
    print_header("cocroutines");
    coroutine1::execute();
    coroutine_iterator::execute();
}
