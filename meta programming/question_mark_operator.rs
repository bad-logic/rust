// -----------------------------------------
//      Question Mark Operator
//          - unwraps/grab the inner value of Ok/Some Variant
// -----------------------------------------

use std::num::ParseIntError;


fn parse_str(input:&str) -> Result<i32,ParseIntError>{
    let integer = input.parse::<i32>()?;
    println!("input: {:?}, integer: {:?}",input,integer);
    Ok(integer)
}

fn divisorR(divident:f64,divisor:f64) -> Result<f64,String>{
    let answer = match divisor{
        0.0 => Err(String::from("Error: Division by zero")),
        _ => Ok(divident/divisor)
    };
    println!("answer {:?}",answer);

    // ownership of answer is moved if one of the result from match is heap allocated
    // here error is type string which is heap allocated
    let correct = answer?; 
    println!("correct: {}",correct);
    Ok(correct)
}

fn divisorO(divident:f64,divisor:f64) -> Option<f64>{
    let answer = match divisor{
        0.0 => None,
        _ => Some(divident/divisor)
    };
    println!("answer {:?}",answer);

    let correct = answer?; 
    println!("answer {:?}",answer);
    println!("correct: {}",correct);
    Some(correct)
}



fn main(){

    let some_vec = vec!["123","avc","12d","34.34"];
    for i in some_vec.iter(){
        println!("{:?}",parse_str(i));
    }

    println!();

    println!("Result: {:?}",divisorR(6.0,0.0));
    println!("Result: {:?}",divisorR(6.0,3.0));

    println!();

    println!("Result: {:?}",divisorO(6.0,0.0));
    println!("Result: {:?}",divisorO(6.0,3.0));

}