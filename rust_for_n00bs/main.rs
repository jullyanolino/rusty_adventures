fn data_types(){
    let int: i8 = -20;
    let uint: u8 = 20;

    let array[i32;5] = [1,2,3,4,5];

    let tuple:(&str, &str, i32) = ("Charles", "Dickens", 1812);
    println!("{} {} was bor in {}", tuple.0, tuple.1, tuple.2);

    let (first_name:&str, last_name:&str, dob:i32) = tuple;
    println!("{} {} was bor in {}", first_name, last_name, dob);   
} 

fn variables(){
    //Produce an "unused variable" warning.
    let _my_integer: u32 = 0;   
    
    fn some_function()->bool{
        return true;
    }
    
    let result:bool = some_function();
    //if result {
    //    println!("Result: {}", result);
    //}
    //preventing #[warn(unused_variables)]
    let _result:bool = some_function();

    //Alone produces #[warn(dead_code)]
    const MY_CONSTANT: i32 = 999;

    //#Conventions
    //{Object:Casing}
    //Variables:snake_case
    //Functions:snake_case
    //Constants:SCREAMING_SNAKE_CASE
    //Types:PascalCase
    //Traits:PascalCase
    //Enums:PascalCase
    
}

fn main(){
    data_types();
}
