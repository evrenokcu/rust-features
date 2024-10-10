mod associated_types;
mod default_generic_type;
mod generics;
use blanket_traits::execute;

use crate::generics_traits::generics::Drink;
mod blanket_traits;
mod bound_in_where;
mod const_generics;
mod downcasting;
mod extension_traits;
mod fully_qualified_syntax;
mod marker_traits;
mod newtype;
mod std_traits;
mod struct_tagging;
mod super_trait;

mod wrapper_structs;
pub(crate) fn generics() {
    generics::add_numbers();
    generics::traits_generics();
    println!("{:#?}", generics::return_trait().brew());
    println!("{:#?}", generics::return_multiple_trait(2).brew());

    associated_types::associated_types();
    default_generic_type::default_generic_type();
    fully_qualified_syntax::fully_qualified_syntax();
    super_trait::super_trait();
    newtype::newtype();
    std_traits::std_traits();
    bound_in_where::bound_in_where();
    downcasting::execute();
    const_generics::execute();
    wrapper_structs::execute();
    extension_traits::execute();
    blanket_traits::execute();
    marker_traits::execute();
    struct_tagging::execute();
}
