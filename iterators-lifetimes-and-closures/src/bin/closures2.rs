fn main(){
    // styles for defining closures

    // 1. explicitly define inputs and outputs type
    let some_closure_1 = |x:u32| -> u32 {x+1};

    // 2. skip the types of inputs and outputs
    // type will be inferred when the closure is called for the first time
    // gives error if it cannot infer the type
    let some_closure_2 = |x| {x+1}; 

    some_closure_2(1);

    // 3. only single statement in the body of the closure
    let some_closure_3 = |x| x+1;

    some_closure_3(1);



    let mut vec_1 = vec![1,2,3];

    let some_closure = || {
        println!("vec 1: {:?}",vec_1);
    };
    println!("vec 1: {:?}",vec_1);
    // vec_1[1] = 1; // cannot update because immutable borrow in line 24
    some_closure();
    vec_1[1] = 1; // scope of immutable borrow ends in line 28 so this is valid


}