#![allow(unused)]

use std::thread::{self, spawn};

fn main() {}

fn test6() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 30,
            height: 50,
        },
        Rectangle {
            width: 10,
            height: 40,
        },
        Rectangle {
            width: 60,
            height: 45,
        },
    ];
    let mut count = 0;
    list.sort_by_key(|r| {
        count += 1;
        println!("current rectangle: {:?}", r);
        r.width
    });
    println!("{:?}", list);
    println!("count: {}", count);
}
fn test5() {
    fn call_once<F: FnOnce() -> String>(f: F) -> String {
        f()
    }

    let closure = || String::from("Hello, world!");
    call_once(closure);
    call_once(closure);
}

fn test4() {
    let s = String::from("Hello, world!");
    let closure = || s;
    let s = closure();
    println!("The string is {}", s);
}
fn test3() {
    let mut list = vec![34, 50, 25, 100, 65];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || {
        println!("Before calling closure: {list:?}");
        list.push(1000);
        println!("After calling closure: {list:?}");
    })
    .join()
    .unwrap();

    // println!("After defining closure: {list:?}"); // Error: cannot borrow `list` as mutable because it is also borrowed as immutable
}

fn test2() {
    let mut list = vec![34, 50, 25, 100, 65];
    println!("Before defining closure: {list:?}");

    let mut borrows_mut = || list.push(1000);
    borrows_mut();
    println!("After calling closure: {list:?}");
}

struct Anomy<'a> {
    inner: &'a mut Vec<i32>,
}

impl<'a> Anomy<'a> {
    fn new(inner: &mut Vec<i32>) -> Anomy {
        Anomy { inner }
    }

    fn call(&mut self) {
        println!("Before calling closure: {:?}", &self.inner);
        self.inner.push(1000);
    }
}

fn test1() {
    let list = vec![34, 50, 25, 100, 65];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From borrows: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
