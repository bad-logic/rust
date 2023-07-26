
#[derive(Debug)] // ensure enum uses debug trait that is used for printing purposes
enum Value {
 Integer(i32),
 Float(f32)
}

fn main(){
    let some_val: Vec<Value> = vec![Value::Integer(12),Value::Float(3.4)];

    println!("The value of the integer is {:?} and float is {:?}", some_val[0],some_val[1]);

    for i in some_val.iter(){
        match i {
            Value::Integer(num) => println!("{} is of integer type",num),
            Value::Float(num) => println!("{} is of float type",num),
        }
    }
}