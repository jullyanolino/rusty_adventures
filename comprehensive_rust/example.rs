fn main(){
    let mut x: i32 = 6;
    print!("{x}");
    while x != 1 {
        if x % 2 = 0 {
            x = x / 2;
        } else {
            print!(" -> {x}");
        }
    }
    println!();
}