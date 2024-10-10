#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
//! # samples crate

//! this is a crate to practise rust
mod closures;
mod concurrency;
mod constants;
#[cfg(feature = "coroutine")]
mod coroutine_feature;
mod generics_traits;
mod global_state;
mod iterators;
mod matches;
mod old_main;
mod patterns;
pub mod prelude;
mod reference_objects;
mod types;
pub(crate) fn print_header(s: &str) {
    println!(
        "===================================={}===================================",
        s
    )
}
pub(crate) fn print_sub_header(s: &str) {
    println!("{}=======================================", s)
}
pub fn execute() {
    old_main::old_main();
    concurrency::concurrency();
    generics_traits::generics();
    constants::constants();
    types::types();
    matches::execute();
    closures::execute();
    iterators::execute();
    global_state::execute();
    patterns::patterns();
    reference_objects::execute();
    #[cfg(feature = "coroutine")]
    coroutine_feature::execute();
}
