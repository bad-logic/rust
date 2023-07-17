

fn stack_function(mut var:i32){
    var = 56;
    println!("var: {}",var);
}

fn heap_function(var:&mut Vec<i32>){
    var.push(50);
    println!("var: {:?}",var);
}



fn main(){
    let stack_num:i32 = 32; // stored in stack
    let mut heap_vec: Vec<i32> = vec![4,5,6]; // stored in heap

    stack_function(stack_num);
    println!("The value inside the main for stack_num: {}", stack_num);

    // heap_function(heap_vec); // var: Vec<i32> = heap_vec move occurs ownership changed
    // heap_vec is no longer valid since it is out of scope and will be dropped

    // change function signature of heap_function 
    // to borrow instead of owning value
    heap_function(&mut heap_vec);

    println!("The value inside the main for heap_vec: {:?}",heap_vec);

    let some_vec: Vec<i32> = vec![4,5,6];// some_vec owns [4,5,6]
    let ref1: Vec<i32> = some_vec;// changes ownership of [4,5,6] to ref1
    let ref2: &Vec<i32> = &ref1;// borrows value from ref1 but does not change the ownership
}