/*
    References Rules:
        - One mutable reference in a scope
        - Many immutable references in a scope
        - Mutable and immutable cannot coexist in same scope
        - Scope of a reference
        - Data should not change when immutable references are in scope

*/
fn main(){

    // 1. cannot have more than one mutable references
    let mut heap_num:Vec<i32> = vec![4,5,6];
    let ref1:&mut Vec<i32> = &mut heap_num;
    // let ref2:&mut Vec<i32> = &mut heap_num; 
    // println!("ref1: {:?}, ref2: {:?}",ref1,ref2);

    // 2. can have multiple immutable references
    let mut heap_num:Vec<i32> = vec![4,6,8];
    let ref1:&Vec<i32> = &heap_num;
    let ref2:&Vec<i32>  = &heap_num;
    println!("ref1: {:?}, ref2: {:?}",ref1,ref2);

    // 3. mutable and immutable cannot coexist
    let mut heap_num:Vec<i32> = vec![1,2,3,4,5];
    // let ref1:&Vec<i32> = &heap_num;
    // let ref2:&Vec<i32> = &heap_num;
    let ref3:&mut Vec<i32> = &mut heap_num;
    // println!("ref1: {:?}, ref2: {:?}, ref3 {:?}",ref1,ref2,ref3);

    // 4. Scope of a reference
    // starts from where it is first used
    // and ends where it is last used
    let mut heap_num:Vec<i32> = vec![1,2,3,4,5];
    let ref1:&Vec<i32> = &heap_num;// scope of ref1 starts here
    let ref2:&Vec<i32> = &heap_num;// scope of ref2 starts here
    println!("ref1: {:?}, ref2: {:?}",ref1,ref2);// scope of ref1 and ref2 ends here

    let ref3: &mut Vec<i32> = &mut heap_num;
    ref3.pop();

    println!("ref3: {:?}",ref3);


    // 5. Data should not change when immutable references are in scope
    let mut heap_num:Vec<i32> = vec![1,2,3,4,5];
    let ref1:&Vec<i32> = &heap_num;// scope of ref1 starts here
    let ref2:&Vec<i32> = &heap_num; // scope of ref2 starts here
    // heap_num.pop();// changing data when ref1 and ref2 immutable references are in scope
    println!("ref1: {:?}, ref2: {:?}",ref1,ref2);// scope of ref1 and ref2 ends here



}