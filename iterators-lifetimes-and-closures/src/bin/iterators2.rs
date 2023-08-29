//------------------------------------------
//          ITERATORS
//------------------------------------------

fn main(){
    let mut some_vec:Vec<i32> = vec![1,2,3,4,5,6];

    let filtered_values = some_vec.iter().filter(|&x| *x >= 5).collect::<Vec<&i32>>();
    println!("filtered values {:?}",filtered_values);
    println!("original vector {:?}",some_vec);

    let a: Vec<i32> = some_vec.clone();

    let filtered_values = some_vec.into_iter().filter(|&x| x >= 5).collect::<Vec<i32>>();
    println!("filtered values {:?}",filtered_values);
    // some_vec is invalid since into_iter moves the ownership
    // println!("original vector {:?}",some_vec); 

    let mut mapped_values = a.iter().map(|x| 2 * *x).collect::<Vec<i32>>();
    println!("The mapped values {:?}",mapped_values);


    let mut mapped_values = a.iter().map(|x| 2 * *x).filter(|x| *x > 6).collect::<Vec<i32>>();
    println!("The mapped values {:?}",mapped_values);


}