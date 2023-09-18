use std::fs::read_dir;
use std::io;
use crate::model_test_kub::Login;

mod model_test_kub;

fn main() {
    let a = 1;
    let mut b = 2;
    let mut username = String::new();
    let mut password = String::new();

    println!("{}", a + b);
    b = 10;
    println!("Enter username:");
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim().to_string();

    println!("Enter password:");
    io::stdin().read_line(&mut password).expect("Failed to read line");
    let password = password.trim().to_string();

    let login_data = Login {
        username: username.clone(),
        password: password.clone(),
    };

    print!("hello {} password {} " ,login_data.username , login_data.password);

    if a > b { println!("Hello, world! B {}", b); } else if a == b { println!("Hello, world! A {} B {}", a, b); } else { println!("Hello, world! A {} ", a); }
    println!("{:?}", fn_test(a, b, "x"));

    for entry in read_dir("./").expect("Failed to read directory") {
        if let Ok(entry) = entry {
            println!("{:?}", entry);
        }
    }
    
    'loop1: for i in 0..10 {
        println!("i {}", i);
        for j in 0..10  {
            println!("j {}", j);
            if j==2 { break 'loop1;}

        }
    }

    //memory
    let mut test = String::from("HELLO");
    hello(&mut test);
    // hello2(test);
    let test_z = &test;
    hello2(test_z);
    println!("{}",test)





}

fn fn_test(a: i32, b: i32, operation: &str) -> (i32, i32) {
    let answer;
    match operation {
        "+" => { answer = a + b }
        "x" => { answer = a * b }
        _ => { answer = a - b }
    }
    return (answer, answer);
}

fn hello(name:&mut String){
    *name= String::from("fdsfsf");
    println!("hello {}", name);
}

fn hello2(name:&String){
    println!("hello2 {}", name);
}


