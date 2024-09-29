use std::cmp::PartialOrd;
fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let max = find_max(&list);
    println!("The maximum number is {}", max);

    let list = vec!['a', 'b', 'c', 'd', 'e'];
    let max = find_max(&list);
    println!("The maximum char is {}", max);
    let s = String::from("Hello, world!");
    let ex = ImportantExcerpt {
        part: "Hello, world!",
    };
    println!("The level is {}", ex.level());
    ex.announce_and_return_part(&s);
}

fn find_max<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

struct Point<T> {
    x: T,
    y: T,
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl ImportantExcerpt<'static> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
