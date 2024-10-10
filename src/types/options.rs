use crate::print_header;

pub(crate) fn execute() {
    print_header("option with ?");
    print_str_option(None);
    print_str_option(Some(String::from("evren")));
}
fn print_str_option(str: Option<String>) -> Option<String> {
    let x = str?;
    let s = x.as_str();
    println!("{x}");
    Some(x)
}
