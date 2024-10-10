use crate::print_header;

mod add_closure;
mod closure_with_move;
pub(crate) fn execute() {
    print_header("Closure");
    add_closure::execute();
    closure_with_move::execute();
}
