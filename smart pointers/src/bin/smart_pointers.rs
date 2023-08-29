//-----------------------------------
// Smart Pointers
//      Box Smart Pointer -> provides way to reference data stored in the heap
//
//-----------------------------------

fn main(){
    // box smart pointer

    //  0.625 stored on heap and the pointer to 0.625 is stored on stack
    let single_value = Box::new(0.625); 

    // stored on stack
    let x = 0.625;

    println!("Are the values equal {} ",x==*single_value);

    let stack_var = 4;
    let stack_ref = &stack_var;

    let heap_var = Box::new(stack_var);

    println!("{} {} {}",stack_var,stack_ref,heap_var);

}