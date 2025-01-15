use std::borrow::Cow;
use std::fmt::Display;

#[derive(Clone)]
struct ListItem<T>
where
    T: Clone,
{
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

#[derive(Clone)]
struct SinglyLinkedList<'a, T>
where
    T: Clone,
{
    head: Cow<'a, ListItem<T>>,
}

impl<T> ListItem<T>
where
    T: Clone,
{
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }
    fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(next)
        } else {
            None
        }
    }
    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() {
            self.next.as_mut().unwrap().mut_tail()
        } else {
            self
        }
    }
    fn data(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> SinglyLinkedList<'_, T>
where
    T: Clone,
{
    fn new(data: T) -> Self {
        SinglyLinkedList {
            head: Cow::Owned(ListItem::new(data)),
        }
    }
    fn append(&self, data: T) -> Self {
        let mut new_list = self.clone();
        let tail = new_list.head.to_mut().mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
        new_list
    }
    fn head(&self) -> &ListItem<T> {
        &self.head
    }
}

fn print_list<T>(list: &SinglyLinkedList<T>)
where
    T: Clone,
    T: Display,
{
    let mut item = list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;
        } else {
            break;
        }
    }
    println!();
}

fn main() {
    let list = SinglyLinkedList::new("head");
    print_list(&list);
    let list = list.append("first");
    print_list(&list);
    let list = list.append("second");
    print_list(&list);
    let list = list.append("third");
    print_list(&list);
    let list = list.append("tail");
    print_list(&list);
}
