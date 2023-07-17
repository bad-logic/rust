//-------------------------------------------
//      Strings
//          - Fixed length strings (&str)
//          - Growable Strings (String)
//---------------------------------------------

fn main(){
    let some_string: &str = "Fixed length string";
    
    let mut growable_string = String::from("This string will grow");

    println!("The string is \"{}\"",growable_string);

    growable_string.push('s');
    println!("The string is \"{}\"",growable_string);

    growable_string.pop();
    println!("The string is \"{}\"",growable_string);

    growable_string.push_str(" which i will use");
    println!("The string is \"{}\"",growable_string);

    println!(
        "Basic functions on strings,
        is_empty(): {},
        length(): {},
        Bytes(): {},
        contains use: {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use")
    );

    growable_string.push_str("   ");
    let str_len =growable_string.trim().len();

    let number = 6;
    let num_str = number.to_string();

    println!("Is the number a string: {}",num_str=="6");

    let some_char: char = 'a';
    let char_str:String = some_char.to_string();

    let my_name:String = "Marley".to_string();

    let empty_string = String::new();

    println!("Length is {}", empty_string.len());

    let s1:String = "Marley".to_string();
    let s2:String = "Bob".to_string();

    let s3:String = format!("{} {}",s2,s1);

    println!("{}",s3);
}