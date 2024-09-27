fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str(", world!");
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // error: value borrowed here after move
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2); // error: value borrowed here after move
    println!("{}", s3);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    println!("change before: {}", s);
    change(&mut s);
    println!("change after: {}", s);

    let mut s = String::from("hello");
    let _r1 = &mut s;
    // let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    // println!("{}, {}", r1, r2); // error: borrow of `s` occurs here

    let s = String::from("hello, world!");
    let word = first_word(&s);
    println!("{}", word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("world");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope.
  // But because it does not have ownership of what it refers to, nothing happens.
fn change(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let chars = s.chars();
    for (i, c) in chars.enumerate() {
        if c == ' ' {
            return &s[..i];
        }
    }
    &s[..]
}
