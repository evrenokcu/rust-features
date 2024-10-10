enum CatColor {
    Black,
    White,
}
struct Cat {
    name: String,
    color: CatColor,
}

pub(crate) fn execute() {
    let c = Cat {
        name: "ruhi".to_string(),
        color: CatColor::Black,
    };
    print_if_black(&c);
    let c = Cat {
        name: "sedat".to_string(),
        color: CatColor::White,
    };
    print_if_black(&c);

    let x: Option<Cat> = None;
}

fn print_if_black(c: &Cat) {
    match c {
        Cat {
            name,
            color: CatColor::Black,
        } => {
            println!("{name} is black");
        }
        Cat { name, color: _ } => println!("{name} is not black"),
    };
}
