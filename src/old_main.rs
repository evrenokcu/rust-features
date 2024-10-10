use std::{collections::HashMap, fmt::Display, vec};

pub(crate) fn old_main() {
    let coffee = Coffee {
        id: 1,
        name: String::from("evren"),
        temp: TempCategory::COLD(Some(3)),
    };
    let coffee2 = Coffee {
        id: 2,
        name: String::from("evren okcu"),
        temp: TempCategory::COLD(None),
    };
    let mut coffee3 = Coffee { id: 3, ..coffee };
    //println!("{}", &coffee);
    println!("{:?}", &coffee2);

    change_coffee(&mut coffee3);

    let orders = [coffee2, coffee3];
    println!("{:?}", orders);

    slices();
    value_by_ref();

    let x = if true { 2 } else { 3 };

    println!("loop:{}", return_from_loop());
    println!("while:{}", return_from_while());
    println!("mutable loop:{:#?}", from_mutable_loop());
    println!(
        "mutable vector:{:#?}",
        from_mutable_vector(&mut vec![1, 2, 3])
    );
    pattern_matching();
    collections();
    iterators();
    traits();
}

fn traits() {
    let cof = Coffee {
        id: 1,
        name: "evren".to_string(),
        temp: TempCategory::HOT(Some(50)),
    };
    let tea = Tea {};

    brew_drink(&cof);
    brew_drink(&tea);
}
fn brew_drink(drink: &impl Brew) {
    println!("{}", drink.brew());
}
trait Brew {
    fn brew(&self) -> String {
        String::from("brewed!")
    }
}
trait Drink: Brew {}
impl Brew for Coffee {}
struct Tea {}
impl Brew for Tea {
    fn brew(&self) -> String {
        String::from("tea brewed")
    }
}
fn iterators() {
    array_iterator();
    hashmap_iterator();
}

fn hashmap_iterator() {
    let template = Coffee {
        id: 1,
        name: String::from("evren"),
        temp: TempCategory::HOT(Some(23)),
    };

    let coffee1 = Coffee { ..template.clone() };
    let coffee2 = Coffee { id: 2, ..template };
    let coffees = HashMap::from([(1, coffee1), (2, coffee2)]);
    let x: Vec<Coffee> = coffees
        .into_iter()
        //.iter()
        .filter(|(k, v)| *k == 1)
        .map(|(k, v)| v)
        //.cloned()
        .collect();

    println!("filtered hashmap:{:#?}", x);
}

fn array_iterator() {
    let array = [1, 2, 3];

    let x = array.iter();
    for item in x {
        println!("{:#?}", item);
    }

    let template = Coffee {
        id: 1,
        name: String::from("evren"),
        temp: TempCategory::HOT(Some(23)),
    };

    let coffee1 = Coffee { ..template.clone() };
    let coffee2 = Coffee { id: 2, ..template };
    let coffees = [coffee1, coffee2];
    let sum: i8 = coffees.iter().map(|c| c.id).sum();
    println!("Sum:{}", sum);
    let sum2: i8 = coffees.iter().map(|c| c.id).sum();
    println!("Sum:{}", sum2);
}

fn collections() {
    let mut hashmap = HashMap::from([(1, "evren"), (2, "okcu")]);
    println!("{:#?}", hashmap);
    println!("{:#?}", hashmap.entry(1))
}

fn pattern_matching() {
    let category = TempCategory::COLD(Some(2));
    //let category = TempCategory::HOT(Some(120));
    if let TempCategory::COLD(degrees) = category {
        println!("cold:{}", degrees.unwrap())
    }
}
fn from_mutable_vector<'a>(vector: &'a mut Vec<i32>) -> &'a Vec<i32> {
    for item in vector.iter_mut() {
        *item = *item + 1
    }
    vector
}

fn from_mutable_loop() -> [i32; 3] {
    let mut array = [1, 2, 3];
    for x in &mut array {
        let val = *x;
        *x = val * 2;
    }
    array
}
fn return_from_while() -> i32 {
    let mut x = 0;
    while x < 10 {
        x += 1;
    }
    x
}
fn return_from_loop() -> i32 {
    let mut x = 0;
    let y = loop {
        x += 5;
        if x > 30 {
            break x;
        }
    };
    y
}

fn value_by_ref() {
    let mut x = 2;
    change_value(&mut x);
    println!("value of x:{}", &x);
}

fn change_value(x: &mut i32) {
    *x = 34;
}

fn slices() {
    let name = String::from("evren");
    let slice = &name[0..1];

    println!("{}", slice);

    let mut numbers = [1, 2, 3, 4];
    let number_slice = &mut numbers[3..4];
    number_slice[0] = 34;
    println!("slice:{:?}", &number_slice);
    println!("array:{:?}", &numbers);
}

#[derive(Debug, Clone)]
struct Coffee {
    id: i8,
    name: String,
    temp: TempCategory,
}

impl Display for Coffee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{:?}", self.id, self.name, self.temp)
    }
}

#[derive(Debug, Clone)]
enum TempCategory {
    HOT(Option<i8>),
    COLD(Option<i8>),
}

fn change_coffee(coffee: &mut Coffee) {
    coffee.name = "okcu".to_string();
}
