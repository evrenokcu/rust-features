use crate::print_header;

mod new_type;
pub(crate) fn execute() {
    print_header("new type pattern");
    new_type::execute();
}
