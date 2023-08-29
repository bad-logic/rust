fn main(){
    let my_information: (&str,i32)= ("salary",40_000);
    println!("{} is equal to {}", my_information.0,my_information.1);

    println!("Another way of printing the whole tuple is {:?}",my_information);

    let (salary, salary_value) = my_information;

    let salary:&str = my_information.0;
    let salary_value:i32 = my_information.1;

    let nested_tuple: (i32,f64,(i32,i32),&str) = (4,5.0,(3,2),"Hello ");
    let element = nested_tuple.2.0;
    let empty_tuple:()= {};
    println!("{:?} {} {:?}",nested_tuple,element,empty_tuple);

}