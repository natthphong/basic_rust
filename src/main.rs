#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
use std::fs::read_dir;

use std::io;
use std::sync::Arc;
use axum::body::{ HttpBody};

use axum::response::{IntoResponse};
use axum::{Json, Router};
use axum::http::{HeaderValue, Method};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::routing::{get, post};
use sqlx::{PgPool, Pool, Postgres, Row};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use crate::handler::user_handler;
use crate::model::Login;
use crate::model::interface::{GradeResult, Speaking};

mod model;
mod handler;

pub struct AppConfig {
    pool:Pool<Postgres>
}

#[tokio::main]
async  fn main() {

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&"postgresql://admin:password@localhost/ksas")
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

   let state  =  Arc::new(AppConfig{pool:pool.clone()});
    let app = Router::new()
        .route("/", get(user_handler::health_checker_handler))
        .route("/api", get(user_handler::health_checker_handler))
        .route("/api/user/:id", get(user_handler::get_customer_by_id_handler))
        .route("/api/user/all", get(user_handler::get_all_customers_handler))
        .route("/api/user/create", post(user_handler::create_customer_handler))
        .route("/api/user/update", post(user_handler::update_customer_handler))
        .route("/api/user/delete/:id", post(user_handler::delete_customer_handler))
        .with_state(state);


    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();


}



#[test]
#[ignore]
fn print_prior(){
    let matrix: [[&str; 35]; 7] = [
        ["O","O","O","O","O" , " ", " ", " "," ", "O" , " "," "," "," " , " "," ", "O"," "," "," "," "," ","O"," "," "," "," "," "," ","O","O","O"," "," "," "],
        ["O"," "," "," ","O" , " ", " ", " "," ", "O" , "O","O","O"," " , " ", " ","O"," "," "," "," ","O"," ","O"," "," "," "," "," ","O"," "," ","O"," "," "],
        ["O"," "," "," ","O" , " ", " ", " "," ", "O" , " "," ","O"," " , " ", " ","O"," "," "," ","O"," "," "," ","O"," "," "," "," ","O"," "," ","O",""," "],
        ["O","O","O","O","O" , " ", " ", " "," ", "O" , " "," "," "," " , " ", " ","O"," "," ","O"," "," "," "," "," ","O"," "," "," ","O","O","O"," "," "," "],
        ["O"," "," "," "," " , " ", " ", " "," ", "O" , " "," "," "," " , " ", " ","O"," "," "," ","O"," "," "," ","O"," "," "," "," ","O"," ","O"," "," "," "],
        ["O"," "," "," "," " , " ", " ", " "," ", "O" , " "," "," "," " , " ", " ","O"," "," "," "," ","O"," ","O"," "," "," "," "," ","O"," "," ","O"," "," "],
        ["O"," "," "," "," " , " ", " ", " "," ", "O" , " "," "," "," " , " ", " ","O"," "," "," "," "," ","O"," "," "," "," "," "," ","O"," "," "," ","O"," "]
    ];
    for row in matrix.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}
#[test]
#[ignore]
fn capacity(){
    println!();
    let models = vec!["iPhone15plus","iPhone15pro","iPhone15promax"];
    let b= vec![6820,7240,7030];
    let per = vec![0.008, 0.4, 18.0];
    for i in 0..models.len() {
        let mut battery:f64 = b[i] as f64;
        let per_rate:f64 = per[i];
        let mut round:f64 = 0.0;
        round =battery/ (b[i] as f64 * (per_rate / 100.0)) ;
        // println!("อัตราการลด {} ต่อหน่วย", (b[i] as f64 * (per_rate / 100.0)));
        if models[i] == "iPhone15plus" {
            println!("{} ใช้งานได้ {} วินาที", models[i], round);
        } else if models[i] == "iPhone15pro" {
            println!("{} ใช้งานได้ {} นาที", models[i], round);
        } else {
            println!("{} ใช้งานได้ {} ชั่วโมง", models[i], round);
        }
    }

}


fn test_basic(){
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

    let login_data = Login::new(username, password);

    print!("hello {} password {} ", login_data.get_username(), login_data.get_password());

    login_data.speak();

    if a > b { println!("Hello, world! B {}", b); } else if a == b { println!("Hello, world! A {} B {}", a, b); } else { println!("Hello, world! A {} ", a); }
    println!("{:?}", fn_test(a, b, "x"));

    for entry in read_dir("./").expect("Failed to read directory") {
        if let Ok(entry) = entry {
            println!("{:?}", entry);
        }
    }

    'loop1: for i in 0..10 {
        println!("i {}", i);
        for j in 0..10 {
            println!("j {}", j);
            if j == 2 { break 'loop1; }
        }
    }

    //memory
    let mut test = String::from("HELLO");
    hello(&mut test);
    // hello2(test);
    let test_z = &test;
    hello2(test_z);
    println!("{}", test);

    let mut str = enum_test(1);
    println!("str {}", str);
    str = enum_test(2);
    println!("str {}", str);
    let z = enum_test(1);
    match z {
        GradeResult::Value(v) => println!("{}", v),
        GradeResult::Error(e) => println!("{}", e),
    }
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

fn hello(name: &mut String) {
    *name = String::from("fdsfsf");
    println!("hello {}", name);
}

fn hello2(name: &String) {
    println!("hello2 {}", name);
}


fn enum_test(a: i32) -> GradeResult {
    if a == 1 {
        return GradeResult::Error("mai hai pen one wa".to_string());
    }
    return GradeResult::Value("OK BRO".to_string());
}

impl Speaking for Login {
    fn speak(&self) {
        println!("SPEAK {}", self.get_username())
    }
}
