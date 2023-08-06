//----------------------------------------
//      as_ref() and partial move in option and result
//----------------------------------------


fn main(){
    let some_option:Option<String> = Some("Alice".to_owned());
    // match some_option {
    //     // some_option value is unwraped and its ownership changed
    //     Some(inner_value) => println!("{}",inner_value),
    //     None => println!("No name provided")
    // }

    // println!("{:?}",some_option); // error ownership changed

    // solution

    // match some_option {
    //     Some(ref inner_value) => println!("{}",inner_value),
    //     None => println!("No name provided")
    // }

    match &some_option {
        Some(inner_value) => println!("{}",inner_value),
        None => println!("No name provided")
    }

    println!("{:?}",some_option);

    let some_1: &Option<String> = &some_option;
    let some_2: Option<&String> = some_option.as_ref();

    try_me(some_option.as_ref());
    println!("{:?}", some_option);

}

fn try_me(options_name:Option<&String>){
    match options_name{
        Some(inner_value) => println!("{}",inner_value),
        None => println!("No name provided")
    }
}