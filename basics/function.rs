

fn main(){
    basic_fn();
    function_with_inputs("Bob",40_000);

    let full_name:&str = "Bob Marley";
    let salary_info:i32 = 50_000;
    function_with_inputs(full_name,salary_info);
    let answer:i32 = function_with_inputs_and_outputs(4,5);
    println!("The answer of multiplication is {}",answer);

    let ( prod, sum, diff) = function_with_inputs_and_multiple_outputs(10,50);
    println!("Product = {} , sum = {}, difference= {}", prod,sum,diff);

    let result = function_with_inputs_and_multiple_outputs(10,50);
    println!("Product = {} , sum = {}, difference= {}", result.0,result.1,result.2);

    let full_name: String = {
        let first_name = "bob";
        let last_name = "marley";

        // no semicolon at end means that is the returning value
        format!("{} {}",first_name,last_name)
    };

    println!("{}",full_name);

    // taking inputs from the user
    let mut user_input:String = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read input.");

    let user_input: f64 = scalar .trim().parse().expect("invalid input");

    println!("user input: {}",user_input);

}

fn basic_fn(){
    println!("this is a basic function");
}

fn function_with_inputs(name: &str, salary:i32){
    println!("The name is {} and salary is {}",name,salary);
}

fn function_with_inputs_and_outputs(num1: i32,num2:i32) -> i32{
    num1 * num2
    // return num1 * num2;
}

fn function_with_inputs_and_multiple_outputs(num1: i32,num2:i32) -> (i32,i32,i32){
    (num1 * num2, num1 + num2, num1 - num2)
}