//-----------------------------
// Arrays
// Syntax: [type;length]
//

fn main(){
    let mut number_array:[i32;5] = [4,5,6,7,9];

    println!("{}",number_array[0]);

    println!("{:?}",number_array);

    number_array[4] = 5;
    println!("{:?}",number_array);

    let array_with_same_element:[i32;10] = [0;10];
    let mut string_array_1:[&str;3] = ["apple","tomato","grapes"];

    let string_array_2:[&str;6]=["unknown";6];
    string_array_1[0] = "hello there";


    let char_array:[char;5]=['a','p','p','l','e'];

    let mut number_array_1:[i32;5] = [4,5,6,8,9];

    let subset_array:&[i32] = &number_array_1[0..=3];

    println!("{:?} {:?} {:?} {:?} {:?} {:?}",
    array_with_same_element,
    string_array_1,
    string_array_2,
    char_array,
    number_array_1,
    subset_array );

    println!("The length of number_array_1 is {}",number_array_1.len());
    println!("The array is occupying {} bytes",std::mem::size_of_val(&number_array));

    let check_index: Option<&i32> = number_array_1.get(100);

    println!("{:?}",check_index);

}