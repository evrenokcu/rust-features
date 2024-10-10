use crate::print_header;

mod options;
mod phantom_data;
pub(crate) fn types() {
    print_header("types");
    phantom_data::phantom();
    options::execute();
}
