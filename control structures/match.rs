//--------------------------------
//      Match
//--------------------------------

fn main(){
    let some_number:i32 = 100;

    match some_number {
        0 => println!("The number is 0"),
        1 => println!("The number is 1"),
        2|3 => println!("The number is 2 or 3"),
        4..=100 => println!("The number is between 4 and 100"),
        _ => println!("The number is greater than 100")
    }

}