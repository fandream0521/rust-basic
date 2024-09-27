use std::i32;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`
    // println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    let guess: u32 = "33".parse().expect("Not a number!");
    println!("guess: {}", guess);

    println!("the max value of u32: {}", u32::MAX);
    println!("the min value of u32: {}", u32::MIN);
    println!("the max value of u64: {}", u64::MAX);
    println!("the min value of u64: {}", u64::MIN);
    println!("the bits of u32: {}", u32::BITS);

    let overflow_x = i32::MAX.overflowing_add(1);
    println!("overflow_x: {:?}", overflow_x);

    let a = 65;
    let b = 20;

    let c1 = a + b;
    let c2 = a - b;
    let c3 = a * b;
    let c4 = a / b;
    let c5 = a % b;

    println!("c1: {}", c1);
    println!("c2: {}", c2);
    println!("c3: {}", c3);
    println!("c4: {}", c4);
    println!("c5: {}", c5);

    let tuple = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
    let (a, .., b) = tuple;
    println!("a: {}, b: {}", a, b);

    let arr = [1, 2, 3, 4, 5];
    let [a, .., b] = arr;
    println!("a: {}, b: {}", a, b);
    let [.., a, b] = arr;
    println!("a: {}, b: {}", a, b);

    let index = String::from("1");

    let index = index.trim().parse::<usize>().expect("Not a number!");
    let element = arr[index];
    println!("element: {}", element);

    another_function();

    print_labeled_value(10, 'm');

    let five = five();
    println!("five: {}", five);

    let number = 3;
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("result: {}", result);

    let mut count = 0;
    'continue_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'continue_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("arr[{}]: {}", index, arr[index]);
        index += 1;
    }

    for element in arr.iter() {
        println!("element: {}", element);
    }

    for (idx, ele) in arr.iter().enumerate() {
        println!("idx: {}, ele: {}", idx, ele);
    }
    println!("====================");

    for i in 0..=10 {
        println!("fab({}): {}", i, fab(i));
    }
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_value(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn fab(mut n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut result = 0;
    while n > 1 {
        result = a + b;
        a = b;
        b = result;
        n -= 1;
    }
    result
}
