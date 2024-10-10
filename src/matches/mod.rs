use crate::print_header;

mod match_partially;
pub(crate) fn execute() {
    print_header("match statements");
    match_partially::execute();
}
