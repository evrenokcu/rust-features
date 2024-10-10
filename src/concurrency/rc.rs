use std::rc::Rc;

use crate::print_header;

pub(crate) fn execute() {
    print_header("Rc<T>");
    sample_1::execute();
}
mod sample_1 {
    use std::rc::Rc;
    pub(super) fn execute() {
        let rc = Rc::new(MyStruct {
            name: "Evren".to_string(),
        });
        let first_ref = rc.clone();
        let second_ref = rc.clone();
        //3 reference counts below; rc itself, first ref and second ref
        println!("reference count={}", Rc::strong_count(&rc));
        print_my_struct(first_ref);
        print_my_struct(second_ref);
    }

    fn print_my_struct(ms: Rc<MyStruct>) {
        println!("{:?}", ms);
    }

    #[derive(Debug)]
    struct MyStruct {
        name: String,
    }
}
