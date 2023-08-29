//```````````````````````````````````````````
// Result enum
//```````````````````````````````````````````

// enum Result<T,E>{
//     Ok(T),
//     Err(E)
// }

fn division(divident:f64, divisor:f64)-> Result<f64,String>{
    // if divisor == 0.0{
    //     Err(String::from("Error: Division by zero"))
    // }else{
    //     Ok(divident / divisor)
    // }

    match divisor{
        0.0 => Err(String::from("Error: Division by zero")),
        _ => Ok(divident / divisor)
    }
}

fn main(){

    println!("{:?}",division(9.0,3.0));
    println!("{:?}", division(4.0,0.0));
    println!("{:?}", division(0.0,10.0));

    let some_vec:Vec<i32> = vec![5,5,2,0,13,7,9,8];

    let result = match some_vec.get(15){
        Some(a) => Ok(a),
        None => Err("The value does not exist.")
    };

    println!("The result is {:?}",result);
}