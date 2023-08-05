//------------------------------------------
//          ITERATORS
//------------------------------------------

fn main(){
    let some_vec:Vec<i32> = vec![1,2,3,4,5,6,7,8];
    let mut iter = some_vec.iter();

    println!("The vector: {:?}",some_vec);
    println!("The iterator: {:?}",iter);

    println!("{:?}",iter.next());
    println!("{:?}",iter.next().unwrap());

    let a:Vec<i32> = vec![0,1,2,3,4,5];

    let mut check:bool = a.iter().any(|&x| x > 0 );
    println!("check {}",check);

    let mut check:bool = a.iter().all(|&x| x > 0 );
    println!("check {}",check);

    let value = a.iter().find(|&&x| x > 0 );
    println!("The first element greater than 0 is {}",value.unwrap());

    let index = a.iter().position(|&x| x > 3);
    println!("The index of first element greater than 3 is {}",index.unwrap());

    let index = a.iter().rposition(|&x| x > 3);
    println!("The index of last element greater than 3 is {}",index.unwrap());

    let max = a.iter().max();
    println!("The max value is {}",max.unwrap());

    let min = a.iter().min();
    println!("The min value is {}",min.unwrap());

    let mut rev = a.iter().rev();
    println!("The rev vector is  {:?}",rev);// Rev {iter:Iter([0,1,2,3,4,5])}
    println!("first value is {:?}",rev.next()); // Some(5)

}