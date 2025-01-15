use std::cell::RefCell;
use std::rc::Rc;

struct ListItem<T> {
    prev: Option<ItemRef<T>>,
    data: Box<T>,
    next: Option<ItemRef<T>>,
}

type ItemRef<T> = Rc<RefCell<ListItem<T>>>;

struct DoublyLinkedList<T> {
    head: ItemRef<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            prev: None,
            data: Box::new(data),
            next: None,
        }
    }
    fn data(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> DoublyLinkedList<T> {
    fn new(data: T) -> Self {
        DoublyLinkedList {
            head: Rc::new(RefCell::new(ListItem::new(data))),
        }
    }
    fn append(&mut self, data: T) {
        let tail = Self::find_tail(self.head.clone());
        let new_item = Rc::new(RefCell::new(ListItem::new(data)));
        new_item.borrow_mut().prev = Some(tail.clone());
        tail.borrow_mut().next = Some(new_item);
    }
    fn head(&self) -> ItemRef<T> {
        self.head.clone()
    }
    fn tail(&self) -> ItemRef<T> {
        Self::find_tail(self.head())
    }
    fn find_tail(item: ItemRef<T>) -> ItemRef<T> {
        if let Some(next) = &item.borrow().next {
            Self::find_tail(next.clone())
        } else {
            item.clone()
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new("head");
    list.append("first");
    list.append("second");
    list.append("third");
    list.append("tail");

    println!("Print forwards:");
    let mut item = list.head();
    loop {
        println!("item: {}", item.borrow().data());
        if item.borrow().next.is_none() {
            break;
        } else {
            let next_item = item.borrow().next.as_ref().unwrap().clone();
            item = next_item;
        }
    }

    println!();
    println!("Print backards:");
    let mut item = list.tail();
    loop {
        println!("item: {}", item.borrow().data());
        if item.borrow().prev.is_none() {
            break;
        } else {
            let prev_item = item.borrow().prev.as_ref().unwrap().clone();
            item = prev_item;
        }
    }
}
