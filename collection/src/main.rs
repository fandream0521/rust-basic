use std::{collections::HashMap, str::FromStr};

fn main() {
    test3();
}

#[derive(Debug)]
pub enum CellType {
    Int(i32),
    Float(f64),
    Text(String),
}

struct commend {
    cmd_type: String,
    employee: String,
    department: String,
}

impl FromStr for commend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.trim().split_whitespace().collect::<Vec<_>>();
        if v.len() != 4 {
            return Err("invalid commend".to_string());
        }
        Ok(commend {
            cmd_type: v[0].trim().to_string(),
            employee: v[1].trim().to_string(),
            department: v[3].trim().to_string(),
        })
    }
}

fn test3() {
    let mut department_map: HashMap<String, Vec<String>> = HashMap::new();
    println!("Enter commend");
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let cmd: Result<commend, _> = input.parse();
        if let Ok(cmd) = cmd {
            let depart = cmd.department;
            department_map
                .entry(depart)
                .or_insert(Vec::new())
                .push(cmd.employee);
        } else {
            break;
        }
    }

    loop {
        println!("enter depart name to search");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input.eq("exit") {
            break;
        }
        let employees = department_map.get(input);
        if let Some(employees) = employees {
            println!("Employee in department {:?}:", input);
            for employee in employees {
                println!("{:?}", employee);
            }
        } else {
            println!("No employee found!");
        }
    }
}

fn test2() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    let first_letter = input.chars().next();
    if first_letter.is_none() {
        println!("Please enter a valid string");
        return;
    }
    let first_letter = first_letter.unwrap();
    let is_vowel = match first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    };
    let append_letter = if is_vowel { 'h' } else { first_letter };
    let input = if is_vowel { input } else { &input[1..] };
    let result = format!("{}-{}ay", input, append_letter);
    println!("{:?}", result);
}

fn test1() {
    let mut vec = Vec::new();
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if let Ok(num) = input.trim().parse::<i32>() {
            vec.push(num);
        } else {
            break;
        }
    }
    println!("{:?}", vec);
    vec.sort();
    println!("{:?}", vec);
    println!("the middle number is {:?}", vec[vec.len() / 2]);

    let mut num_map = HashMap::new();
    for num in vec.iter().copied() {
        let count = num_map.entry(num).or_insert(0);
        *count += 1;
    }

    let v = num_map.iter().max_by(|a, b| a.1.cmp(b.1));
    println!("{:?}", v);
}

fn basic_test() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("{:?}", v2);

    let mut v = Vec::new();
    v.push(1);
    v.push(4);
    v.push(5);
    println!("{:?}", v);

    let v1 = v.pop();
    println!("{:?}", v1);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{:?}", third);

    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    let mut v = vec![100, 32, 57];
    let first = &v[0];

    // v.push(6); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{:?}", first);

    v.push(6);
    println!("{:?}", v);

    for i in &v {
        println!("{:?}", i);
    }

    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    let row = vec![
        CellType::Int(3),
        CellType::Text(String::from("blue")),
        CellType::Float(10.12),
    ];
    println!("{:?}", row);

    let data = "initial contents";
    let s = data.to_string();
    println!("{:?}", s);

    let s = String::from("initial contents");
    println!("{:?}", s);

    let hello = String::from("السلام عليكم");
    println!("{:?}", hello);

    let hello = String::from("Dobrý den");
    println!("{:?}", hello);

    let hello = String::from("Hello");
    println!("{:?}", hello);

    let hello = String::from("שָׁלוֹם");
    println!("{:?}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{:?}", s);
    s.push('l');
    println!("{:?}", s);

    let s1 = String::from("Hello, ");
    let s2 = s1 + "world!";
    println!("{:?}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{:?}", s);

    let s1 = String::from("hello");
    // let h = s1[0]; // error[E0277]: the type `String` cannot be indexed by `{integer}`
    let h = &s1[0..1];
    println!("{:?}", h);

    let s = String::from("नमस्ते");
    println!("{:?}", s.len());
    for c in s.chars() {
        println!("{:?}", c);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}", score);

    for (k, v) in &scores {
        println!("{:?}: {:?}", k, v);
    }
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 22);
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores);
}
