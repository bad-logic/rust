//----------------------------
//  Vectors
//----------------------------

fn main(){
    let mut number_vec: Vec<i32> = vec![4,5,6,8,9,12,11,43,45];

    println!("{}",number_vec[0]);
    println!("{:?}",number_vec);

    number_vec[4] = 5;
    println!("{:?}",number_vec);

    let vector_with_same_elements: Vec<i32> = vec![0;10];

    let mut string_vector_1 : Vec<&str> = vec!["apple","tomato","grapes"];
    let string_vector_2: Vec<&str> = vec!["unknown";6];

    string_vector_1[0]="welcome";

    let char_vector: Vec<char> = vec!['a','p','p','l','e'];

    let subset_vec: &&[i32] = &&number_vec[0..5];
    println!("The subset of values of the subset_vec are {:?}",subset_vec);

    println!("Number of Elements in the number_vec are {}",number_vec.len());

    let check_index: Option<&i32> = number_vec.get(100);

    println!("{:?}",check_index);

    number_vec.push(30);
    number_vec.push(40);
    println!("Elements in the number_vec are {:?}",number_vec);

    number_vec.remove(5);
    println!("Elements in the number_vec are {:?}",number_vec);

    println!("The value of 10 exists in number_vec: {}",number_vec.contains(&10));

}