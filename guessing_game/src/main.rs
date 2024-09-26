use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    // 打印欢迎信息
    println!("Guess the number!");
    println!("Please input your guess.");

    // 生成一个1到100之间的随机数
    let random_num = rand::thread_rng().gen_range(1..101);

    // 无限循环，直到用户猜对数字或输入退出命令
    loop {
        // 读取用户输入的数字，如果用户输入"quit"，则退出循环
        let Some(guess) = read_number() else {
            break;
        };

        // 比较用户输入的数字和随机数
        match random_num.cmp(&guess) {
            Ordering::Less => println!("Too big!"), // 用户输入的数字太大
            Ordering::Greater => println!("Too small!"), // 用户输入的数字太小
            Ordering::Equal => {
                println!("You win!"); // 用户猜对了数字
                break;
            }
        }
    }
}

fn read_number() -> Option<u32> {
    loop {
        let mut guess = String::new();
        // 读取用户输入的一行
        if let Ok(_) = io::stdin().read_line(&mut guess) {
            // 尝试将输入解析为数字
            if let Ok(num) = guess.trim().parse::<u32>() {
                return Some(num);
            } else {
                // 如果用户输入"quit"，则返回None
                if "quit".eq(&guess.trim().to_lowercase()) {
                    return None;
                } else {
                    // 提示用户输入一个数字
                    println!("Please input a number!");
                }
            }
        }
    }
}
