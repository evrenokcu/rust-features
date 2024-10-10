mod borrow_str;

mod as_ref;

mod borrow_trait;
mod cow;
mod deref;
pub(crate) fn std_traits() {
    crate::print_header("std traits");
    partial_equal();
    overrided_partial_equal();
    partial_ord();
    deref::deref();
    as_ref::as_ref();
    borrow_trait::borrow();
    borrow_str::borrow();
    cow::cow();
}

#[derive(Debug)]
struct MyStruct {
    name: String,
    id: i32,
}

impl Default for MyStruct {
    fn default() -> Self {
        Self {
            name: "evren".to_string(),
            id: 1,
        }
    }
}
impl PartialEq for MyStruct {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PartialEq<&str> for MyStruct {
    fn eq(&self, other: &&str) -> bool {
        self.name == *other
    }
}

impl Eq for MyStruct {}

impl PartialOrd for MyStruct {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.id.partial_cmp(&other.id)
    }
}

fn partial_ord() {
    let s1 = MyStruct {
        name: "A".to_string(),
        id: 100,
    };
    let s2 = MyStruct {
        name: "Z".to_string(),
        id: 1,
    };

    println!(
        "Partial Ord {:?} is greater {:?}, result: {}",
        s1,
        s2,
        s1 > s2
    );
}

fn overrided_partial_equal() {
    let my = MyStruct {
        name: "evren".to_string(),
        ..Default::default()
    };
    println!("are equal: {}", my == "evren");
}

fn partial_equal() {
    let my2 = MyStruct {
        id: 2,
        ..Default::default()
    };
    let otherMy2 = MyStruct {
        id: 2,
        ..Default::default()
    };
    println!("are equal:{:?}", my2 == otherMy2);
}

fn default_trait() {
    let my: MyStruct = Default::default();
    let my = MyStruct::default();
    println!("default trait: {:?}", my);

    let my2 = MyStruct {
        id: 2,
        ..Default::default()
    };
    let otherMy2 = MyStruct {
        id: 2,
        ..Default::default()
    };
}
