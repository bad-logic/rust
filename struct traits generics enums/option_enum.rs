//```````````````````````````````````````````
// Option enum
//```````````````````````````````````````````

// enum Option<T>{
//     None,
//     Some(T)
// }

fn square(num: Option<i32>) -> Option<i32>{
    match num {
        Some(number) => Some(number * number),
        None => None
    }
}

fn main(){

    let mut disease : Option<String> = None;

    disease = Some(String::from("diabetes"));

    match disease{
        Some(disease_name) => println!("Disease: {}",disease_name),
        None => println!("No disease")
    }

    let s1: Option<&str> = Some("Some String");
    println!("The value of s1 is {:?} and the value of s1 itself after unwrapping is {:?}",s1,s1.unwrap());

    let number: Option<i32> = Some(6);
    println!("The square of num is {}",square(number).unwrap());

    println!("square of None is {:?}",square(None));
}