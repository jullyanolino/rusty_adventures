#![allow(unused)]

use std::fmt::Alignment;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::ops::Add;

use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;

use crate::io::Error;

use std::thread;
use std::time::Duration;

//First hour
fn first_hour(){
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

    let random_num: i8 = rand::thread_rng().gen_range(1..101);
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
    let mut str1: String = String::new();
    str1.push('A');
    str1.push_str(" word");
    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    let str2: String = str1.replace('A', "Another");
    println!("{}", str2);

    let str3: String = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        print!("{} ", char);
    }

    let str4: &str = "Random string";
    let mut str5: String = str4.to_string();
    println!("str5: {}", str5);

    let byte_arr1: &[u8] = str5.as_bytes();
    let str6: &str = &str5[0..6];
    println!("String legth: {}", str6.len());

    str5.clear();
    println!("str5: {}", str5);

    let str6: String = String::from("Just some");
    let str7: String = String::from(" words");
    let str8 = str6 + &str7;
    for char in str8.bytes(){
        println!("{}", char);
    }

    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("{}", int3_u32);

    enum Day {
        Monday, 
        Tuesday, 
        Wednesday,
        Thursday,
        Friday, 
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone loves Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend {}?", today.is_weekend());

    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1,2,3];
    vec2.push(5);
    println!("1st: {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1){
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;
        println!("{}", &i);
    } 

//    for i in &vec2 {
//        println!("{}", i);
//    }

    println!("Vector length: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());

}

fn get_sum(x: i32, y: i32) -> i32{
    //x + y
    return x + y;
}

//Second hour
fn get_two(x: i32) -> (i32, i32){
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }

    sum
}

fn sum_list_2(list: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for val in list.iter(){
        sum += val;
    }

    sum
}

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn print_str(x: String) {
    println!("A string: {}", x);
}

fn print_return_str(x: String) -> String{
    println!("A string: {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str(" is happy");
    println!("Message: {}", name);
}

fn second_hour(){
    println!("{}",get_sum(3,9));
    let (val_1, val_2) = get_two(1);
    println!("{}, {}", val_1, val_2);

    let num_list: Vec<i32> = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));
    println!("Sum of list = {}", sum_list_2(num_list));

    println!("4 + 23 = {}", get_sum_gen(4, 23));
    println!("4.1 + 23.7 = {}", get_sum_gen(4.1, 23.7));

    let str1: String = String::from("World");
    let str2: String = str1.clone();
    //print_str(str1);
    
    let str3:String = print_return_str(str1);
    println!("str3 = {}", str3);

    let mut str4: String = String::from("Lino");
    change_string(&mut str4);


    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");

    for (k, v) in heroes.iter(){
        println!("{} = {}", k, v);
    }

    println!("Length: {}", heroes.len());

    if heroes.contains_key(&"Batman"){
        let the_batman: Option<&&str> = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob: Customer = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main Street"),
        balance: 345.89
    };

    bob.address = String::from("505 Main Street");

    struct RectangleAux<T, U> {
        lenght: T,
        height: U,
    }

    let rec = RectangleAux {
        lenght: 4,  
        height: 10.5
    };

    const PI: f32 = 3.1415952; 
    trait Shape {
        fn new(lenght: f32, height: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    impl Shape for Rectangle{
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32{
            return self.length * self.width;
        }
    }

    impl Shape for Circle{
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32{
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rectangle area: {}", rec.area());
    println!("Circle area: {}", circ.area());

    // Crates: modules that produce a library or executable
    // Modules: organize and handle privacy
    // Packages: Build, test and share crates
    // Paths: A way of naming an item such as a struct, funcion
    order_food();

    //panic!("Terrible error");

    let path: &str = "lines.txt";
    let output: Result<File, Error> = File::create(path);
    let mut output: File = match output {
        Ok(file) => file,
        Err(error) => 
            panic!("Problem creation file: {:?}", 
                error),
    };
    write!(output, "Just some Random words")
        .expect("Failed to write to file");

    let  input: File = 
        File::open(path).unwrap();
    let buffered: BufReader<File> = 
        BufReader::new(input);

    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }

    let output2: Result<File, Error> = 
        File::create("rand.txt");
    let output2 = match output2{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => 
                match File::create ("rand.txt"){
                    Ok(fc) => fc,
                    Err(e) => panic!("Can't create file: {:?}", error),
                },
        _other_error => 
            panic!("Problem opening file: {:?}", error),
        },
    };

    let mut arr_it: [i32; 4] = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    println!("1st: {:?}", iter1.next());
}

fn main() {
    //first_hour();
    //second_hour();

    //CLOSURE
    // A closure is a function without a name and they are sometimes
    // stored in a variable (They can be used to pass a function into
    // another function)
    // let var_name = |parameters| -> return_type {BODY}
    let can_vote = |age: i32| -> bool {
        age >= 18
    };
    println!("Can vote: {}", can_vote(9));

    let mut samp1: i32 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where T: Fn(i32, i32) -> i32 {
        func(a, b)
    } 
    let sum = |a: i32, b: i32| a+b;
    let prod = |a: i32, b: i32| a*b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

    // ----- SMART POINTERS -----
    // A pointer is an address to a location in memory. We have been
    // using them when we used the reference operator(&) to borrow
    // a value.

    // Strings and vectors are smart pointers. They own
    // data and also have functions for manipulating that data.

    // Smart pointers provide functionality beyond referencing locations
    // in memory. They can be used to track who has ownership of data.
    // Ownership is very important with Rust.

    // ----- BOX -----

    // The Box smart pointer stores data on the heap instead of the stack.
    // All values are stored on the stack by default

    // Stack : Stores values in a last in first out format
    // Data on the stack must have a defined fixed size

    // Heap : When putting data on the heap you request a certain
    // amount of space. The OS finds space available and returns
    // an address for that space called a pointer.

    // A Box is normally used when you have a large amount of data stored
    // on the heap and then you pass pointers to it on the stack.

    let b_int1: Box<i32> = Box::new(10);
    println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self{
            TreeNode { left: None, right: None, key }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self{
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self{
            self.right = Some(Box::new(node));
            self
        }

    }

    let node1: TreeNode<i32> = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));

    // ----- CONCURRENCY -----
    // Concurrent programming envolves executing different blocks of code
    // independently, while parallel programming is when different
    // code executes at the same time. A thread handles scheduling
    // and execution of these blocks of code.

    // Common problems with parallel programming involve :
    // 1. Thread are accessing data in the wrong order
    // 2. Threads are blocked from executing because of confusion
    // over requirements to proceed with execution

    let thread1 = 
        thread::spawn(|| {
            for i in 1..25 {
                println!("Spawned thread: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

    for c in 1..20 {
        println!("Main thread: {}", c);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();

    pub struct Bank {
        balance: f32
    }

    fn withdraw(the_bank: &mut Bank, amt: f32){
        the_bank.balance -= amt;
    }
    let mut bank: Bank = Bank{ balance: 100.00 };
    withdraw(&bank, 5.00);
    println!("Balance: {}", bank.balance);
}