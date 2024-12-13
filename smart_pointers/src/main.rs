use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let c = Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))));
    println!("{c:#?}");

    let b = 5;
    let bb = MyBox::new(b);

    assert_eq!(5, *bb);
}
