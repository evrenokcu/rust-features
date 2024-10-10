use crate::print_header;
mod reference_objects_lib;
pub(crate) fn execute() {
    print_header("reference objects");
    reference_objects_lib::execute();
}
