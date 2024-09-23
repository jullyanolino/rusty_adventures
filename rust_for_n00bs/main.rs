fn main(){
    let int: i8 = -20;
    let uint: u8 = 20;

    let array[i32;5] = [1,2,3,4,5];

    let tuple:(&str, &str, i32) = ("Charles", "Dickens", 1812);
    println!("{} {} was bor in {}", tuple.0, tuple.1, tuple.2);

    let (first_name:&str, last_name:&str, dob:i32) = tuple;
    println!("{} {} was bor in {}", first_name, last_name, dob);
}
