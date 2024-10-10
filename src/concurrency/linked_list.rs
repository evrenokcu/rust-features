use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::print_header;

pub(crate) fn execute() {
    print_header("linked list");
    let mut cats = LinkedList::new("Ruhi");
    cats.append("Sedat");
    cats.append("Zarife");

    // cats.into_iter().for_each(|i| {
    //     let x = i.borrow().d
    //     println!("{}", x.)
    // });
    //while
}

type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

struct ListItem<T> {
    data: ItemData<T>,
    next: Option<ListItemPtr<T>>,
}
impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(data)),
            next: None,
        }
    }
}

struct LinkedList<T> {
    head: ListItemPtr<T>,
    cur_iter: Option<ListItemPtr<T>>,
}
impl<T> LinkedList<T> {
    fn new(data: T) -> Self {
        Self {
            head: Rc::new(RefCell::new(ListItem::new(data))),
            cur_iter: None,
        }
    }

    fn append(&mut self, t: T) {
        let mut next = self.head.clone();
        while next.as_ref().borrow().next.is_some() {
            let n = next.as_ref().borrow().next.as_ref().unwrap().clone();
            next = n;
        }
        next.as_ref().borrow_mut().next = Some(Rc::new(RefCell::new(ListItem::new(t))));
    }
    fn iter(&self) -> Iter<T> {
        Iter {
            next: Some(self.head.clone()),
        }
    }
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: Some(self.head.clone()),
        }
    }
    fn into_iter(&mut self) -> IntoIter<T> {
        IntoIter {
            next: Some(self.head.clone()),
        }
    }
}

// impl<T> Iterator for LinkedList<T> {
//     type Item = ListItemPtr<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match &self.cur_iter.clone() {
//             Some(ptr) => self.cur_iter = ptr.borrow().next.clone(),
//             None => {
//                 self.cur_iter = Some(self.head.clone());
//             }
//         }
//         self.cur_iter.clone()
//     }
// }

// impl<T> Iterator for Iter<T> {
//     type Item = ItemData<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next.clone() {
//             Some(ptr)=>{
//                 self.next.clone_from(source);
//             }
//         }
//     }
// }

struct Iter<T> {
    next: Option<ListItemPtr<T>>,
}
struct IterMut<T> {
    next: Option<ListItemPtr<T>>,
}
struct IntoIter<T> {
    next: Option<ListItemPtr<T>>,
}
