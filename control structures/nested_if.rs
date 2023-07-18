fn main(){
    let mut some_number: String = String::new();
    println!("Enter a number:");

    std::io::stdin().read_line(&mut some_number).expect("failed to read input");

    let some_number:i32 = some_number.trim().parse().expect("input should be a number");

    if some_number!= 0 {
        // bitwise AND odd number have their LSB turned on (i.e. 1)
        // binary of some_number & ...000001 will result to ...000001 if it is odd
        if some_number & 1 == 1 {
            println!("the number is odd");
        }else{
            println!("the number is even");
        }
    }else{
        println!("number cannot be zero");
    }

    // if-let
    let value: &str = if some_number & 1 == 1{
        "odd"
    }else{
       "even"
    };

    println!("number is {}",value);
    
}