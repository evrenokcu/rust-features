use std::borrow::Borrow;

pub(crate) fn borrow() {
    crate::print_header("borrow");
    let mut r = Repository { items: Vec::new() };
    r.add(Item {
        id: 1,
        name: "Evren".to_string(),
    });
    let found = r.get("Evren");
    println!("borrowed by name: {:?}", found);
    let found = r.get(&(1 as i32));
    println!("borrowed by id: {:?}", found)
}

#[derive(Debug)]
struct Item {
    id: i32,
    name: String,
}

struct Repository {
    items: Vec<Item>,
}

impl Repository {
    fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    fn get<T: ?Sized>(&self, t: &T) -> Option<&Item>
    where
        Item: Borrow<T>,
        T: PartialEq,
    {
        self.items.iter().find(|i| (*i).borrow() == t)
    }
}

impl Borrow<str> for Item {
    fn borrow(&self) -> &str {
        let name = &self.name;
        name
    }
}
impl Borrow<i32> for Item {
    fn borrow(&self) -> &i32 {
        &self.id
    }
}
