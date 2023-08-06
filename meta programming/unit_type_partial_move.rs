//---------------------------------------
//      - Unit Type
//      - Expression vs statements
//      - Partial Move
//---------------------------------------


// below functions return unit value
// if nothing is returned then unit value is returned
fn f1() -> () {}

fn f2(){}

fn division(divident:f64,divisor:f64) -> Result<(),String>{
    let answer = match divisor{
        0.0 => {
            println!("Error of division by zero");
            Err(String::from("Error: Division by zero"))
        },
        _ => {
            println!("The division is valid");
            Ok(())
        }
    };
    answer
}

fn square(i:i32)->i32{
    i*i
}

#[derive(Debug)]
struct Student{
    name:String,
    courses: Vec<String>,
    age:u8
}

fn main(){
    // Unit type

    let x:() = ();

    f1();
    f2();

    // variable x has a unit type
    let x:() = {
        println!("hello")
    };

    division(9.0,0.0);
    division(9.0,3.0);

    let x:() = ();
    let y:() = println!("hello world");
    println!("the two variables are equal {}", x == y);

    // Expression vs Statements
    let num = 10;
    let y = {
        // this code block is an expression since it returns value
        // and this expression is assigned to variable y
        let x = String::from("Hello rust");
        x
    };

    square(4); // returns value so expression
    println!("This is a simple statement"); // does not return any thing so statement

    // Partial Move
    let std = Student{
        name:String::from("Bob"),// heap allocated
        courses: vec!["Geometry".to_string(),"Trigonometry".to_string(),"Physics".to_string()],// heap allocated
        age: 22 // stack allocated
    };

    let name = std.name; // ownership changed
    let courses = &std.courses; // borrowed
    let age = std.age; // copy


    // println!("std.name {}",std.name);
    println!("std.courses {:?}",std.courses);
    println!("std.age {}",std.age);


}