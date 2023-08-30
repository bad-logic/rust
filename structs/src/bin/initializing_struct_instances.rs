//-----------------------------------
// 
//          Initializing Struct Instances
// 
//-----------------------------------

mod lib;

fn main(){

    // let new_student = Student{
    //     id:222,
    //     name: "Bob".to_string(),
    //     age:20
    // };

    let new_student = lib::Student::new("Bob12".to_string())
    .unwrap_or_default();

    println!("{:?}", new_student);

    
}