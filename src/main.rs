fn main() {
    let (x, y) = assign_numbers();

    print_two_numbers(x, y);
}
   
fn assign_numbers() -> (i32, i32){
    (1, 5)
}

fn print_two_numbers(x: i32, y: i32){
    println!("x:{}, y:{}",x,y);
}