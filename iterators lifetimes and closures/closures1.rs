
fn division<F: Fn(f32) -> bool> (x:f32,y:f32,f:F){
    if f(y) == true {
        println!("The division result is {}",x /y);
    }else{
        println!("The division is not possible");
    }
}


fn main(){

    let x: i32 = 5;
    // all variables in the code segment in which the closure is defined 
    // are visible to the closure
    let square = || println!("The square of the variable {} is {}", x, x*x);
    square();

    let square = |n: i32| println!("The cube of the variable {} is {}", n, n*n*n);
    square(3);

    let print_info = |general_info:String,name:&str,age:i32 | println!("{}\n\t{}: {}",general_info,name,age);
    
    let general_info = String::from("The details are");
    let (person_name,person_age) = (String::from("Bob"),48);
    
    // here The details are strings ownership is transferred from general_info to 
    // general_info of closure function
    print_info(general_info,&person_name,person_age);

    // due to ownership transfer general_info is no longer valid
    // println!("general info>> {}",general_info);

    let division_status = |y:f32| {if y != 0.0 {true} else {false}};

    division(5.0,10.0,division_status);
    division(5.0,0.0,division_status);
    division(0.0,10.0,division_status);


}

