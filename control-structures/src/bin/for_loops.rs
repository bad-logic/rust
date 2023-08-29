fn main(){
    let vector: Vec<i32> = vec![23,45,67,89,87];
    let mut some_vector: Vec<i32> = vec![23,45,67,89,87];

    for i in 0..5{
        println!("{}",vector[i]);
    }

    for i in vector{ // ownership of vector is moved
        println!("move {}",i);
    }

    // println!("{:?}",vector);
    for i in &some_vector{
        println!("borrow {}",i);
    }

    for i in some_vector.iter(){
        println!("iter {}",i);
    }
    println!("{:?}",some_vector);

    for i in some_vector.iter_mut(){
        *i += 10;
        println!("dereferencing {}",i);
    }

    for i in &mut some_vector{
        *i += 10;
        println!("ref {}",i);
    }
    println!("{:?}",some_vector);

}