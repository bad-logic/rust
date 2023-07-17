//------------------------------------
//  Rust Ownership
//  Rules:
//      - Each value in Rust has a variable that's called its owner.
//      - There can be only one owner at a time.
//      - When the owner goes out of scope, the value will be dropped.
//------------------------------------

// premitive variable type has fixed size cannot grow (integer, float, character, boolean, Array)
// premitive variables are stored in stack

// non-premitive variable type can grow ( String, Vector )
// non-premitive variable type are stored in heap

// copy occurs in premitive variable types 
// move occurs in non-premitive variable types 
// move changes the ownership
// reference leads to borrowing

// Example
// let x: f64 = 32.6; // premitive variable stored in stack
// let y: f64 = x; // copy occurs i.e; value of x is copied over y

// let s1: String = String::from("abc"); // non premitive stored in heap
// let s2: String = s1; // move occurs i.e; owner of string "abc" is changed to s2
// let s3: &String = &s1; // borrowing value without affecting ownership
// let s4: String = s1.clone(); // cloning so that s4 is owner of different string "abc"

fn main(){
    let x: f64 = 32.6;
    let y: f64 = x;
    

    println!("x: {}, y: {}",x,y);

    // s1 is owner of string "abc"
    let s1: String = String::from("abc");

    // ownership changed to s2. hence s1 is invalid
    let s2: String = s1;
    println!(" s2: {}",s2); // valid
    // println!("s1: {}, s2: {}",s1,s2); // error since s1 is invalid

    // borrowing value instead of moving ownership
    let s3: &String = &s2;
    println!("s2: {}, s3: {}",s2,s3); 

    // let vector_1: Vec<i32> = vec![5,6,9,8,7];
    // let vector_2 = vector_1;
    // println!("Vector 1: {:?}, Vector 2: {:?}",vector_1,vector_2);// vector 1 not usable

    let vector_1: Vec<i32> = vec![5,6,9,8,7];
    let vector_2 = &vector_1;
    println!("Vector 1: {:?}, Vector 2: {:?}",vector_1,vector_2); 

    let vect_1: Vec<i32> = vec![5,6,9,8,7];
    let vect_2 = vect_1.clone();
    println!("Vect 1: {:?}, Vect 2: {:?}",vect_1,vect_2); 

    {
        let my_name = String::from("Bob Marley");
    }

    // println!("name: {}",my_name); // out of scope

}