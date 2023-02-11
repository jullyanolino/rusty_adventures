#![allow(unused)]

use std::fmt::Alignment;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("Didn't Receive Input");
    println!("Hello {}! {}", name.trim_end(), greeting);

    const ONE_MIL:u32 = 1_000_000;
    const PI:f32 = 3.141592;
    let age: &str = "38";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} an I want ${}", age, ONE_MIL);

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    let is_true:bool = true;
    let my_grade = 'A';
    let num_1: f32 = 1.111111111111111111;
    println!("f32 : {}", num_1+0.11111111111111111);
    let num_2: f64 = 1.111111111111111111;
    println!("f64 : {}", num_2+0.11111111111111111);
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);
    num_3 += 1;

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);

    let age: i32 =  8;
    if (age >= 1) && (age <= 18){
        println!("Important birthday");
    } else if (age == 21) || (age == 50){
        println!("Important birthday");
    } else if age >= 65 {
        println!("Important birthday");
    } else{
        println!("not an important birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("Important birthday"),
        65..=i32::MAX => println!("Important birthday"),
        _ => println!("Not an important birthday"),
    };

    let my_age = 18;
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can't vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };

    let arr_1: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let arr_2: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx: usize = 0;

    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    } 

    loop_idx = 0;
    while loop_idx < arr_2.len(){
        println!("Arr: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val: {}", val);
    }

    let my_tuple: (u8, String, f64) = (37, "Lino".to_string(), 50_000.00);

    println!("Age: {}", my_tuple.0);
    println!("Name: {}", my_tuple.1);

    let(v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);

    //String 
    //&str
    let mut str0 = String::new("immutable string");

    let mut str1 = String::new();
    str1.push(ch: 'A');
    str1.push_str(String: " word");
    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    let str2: String = str1.replace(from: "A", "Another");
    println!("{}", str2);


}
