pub trait Description {
    fn describe() -> String;
}

fn describe_type<T: Description>() -> String {
    T::describe()
}

struct Dog();
struct Cat();

impl Description for Dog {
    fn describe() -> String {
        "happy little dog".into()
    }
}

impl Description for Cat {
    fn describe() -> String {
        "curious cat".into()
    }
}

fn main() {
    println!("I am a {}", describe_type::<Dog>());
    println!("I am a {}", describe_type::<Cat>());
}
