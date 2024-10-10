use std::{cell::RefCell, rc::Rc};

use crate::print_header;

pub(crate) fn execute() {
    print_header("ref cell");
    sample_1::execute();
    sample_2::execute();
    rc_ref_cel();
    ref_cell_rc();
}

fn rc_ref_cel() {
    print_header("Rc<RefCell<T>>");
    let cat: Rc<RefCell<String>> = Rc::new(RefCell::new("Ruhi".to_string()));
    let cat2 = Rc::clone(&cat);
    {
        let mut x = cat2.borrow_mut();
        x.push_str("evren");
    }

    println!("{:?}", cat2.borrow())
}
fn ref_cell_rc() {
    print_header("RefCell<Rc<T>>");
    let cat = RefCell::new(Rc::new(String::from("Ruhi")));

    *cat.borrow_mut() = Rc::new(String::from("okcu"));
    println!("{:?}", cat)
}
mod sample_1 {
    use std::{cell::RefCell, rc::Rc};

    use crate::print_header;
    pub(super) fn execute() {
        print_header("sample 1");
        let b = Rc::new(Button::new("evren"));

        println!("{:?}", b.get_label());
        let event_handler = b.clone();
        let ui_view = b.clone();
        event_handler.set_label("event handler set");
        println!("{:?}", ui_view.get_label());

        ui_view.set_label("ui vew set the label");
        println!("{:?}", event_handler.get_label());

        ui_view.set_label("ui view set the label again");
        println!("{:?}", event_handler.get_label());
    }
    #[derive(Debug)]
    struct Button {
        label: RefCell<String>,
    }
    impl Button {
        fn new(label: &str) -> Button {
            Button {
                label: RefCell::new(label.to_string()),
            }
        }
        fn set_label(&self, label: &str) {
            *self.label.borrow_mut() = label.to_string()
        }
        fn get_label(&self) -> String {
            self.label.borrow().to_string()
        }
    }
}
mod sample_2 {
    use std::{cell::RefCell, rc::Rc};

    use crate::print_header;
    pub(super) fn execute() {
        print_header("sample 2");
        let n_1 = Node::new(1);
        let n_1_1 = Node::new(2);
        Node::add_child(n_1_1, &n_1);
        let n_1_2 = Node::new(3);
        Node::add_child(n_1_2, &n_1);

        println!("{:#?}", &n_1)
    }
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Self {
                value,
                children: RefCell::new(vec![]),
            })
        }
        fn add_child(val: Rc<Node>, parent: &Rc<Node>) {
            parent.children.borrow_mut().push(val);
        }
    }
}
