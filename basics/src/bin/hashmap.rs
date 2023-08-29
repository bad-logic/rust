

use std::collections::HashMap;

fn main(){
    let mut person:HashMap<&str,i32> = HashMap::new();
    person.insert("bob",40);
    person.insert("ben",40);
    person.insert("tyson",40);
    person.insert("robert",40);

    // get from hash map
    println!("bob's age is {:?}", person.get("bob").unwrap());

    // check if key exists
    if person.contains_key("bob"){
        println!("the key exists");
    }else{
        println!("the key does not exists");
    }

    person.insert("tyson",50); // will replace existing value
    person.entry("robert").or_insert(50); // will make entry if key does not exist already

    for (name, age) in &person{
        println!("the person {} has an age of {}",name,age);
    }

    let some_vector: Vec<i32> = vec![1,2,3,4,2,4,7,8,9,3,3,67,3];
    let mut free_vec: HashMap<i32,u32> = HashMap::new();

    // for i in some_vector.iter(){
    //     if free_vec.contains_key(i) {
    //         let prev_count = free_vec.get(i).unwrap() as &u32;
    //         free_vec.insert(*i, *prev_count + 1);
    //     }else{
    //         free_vec.insert(*i,1);
    //     }
    // }

    for i in some_vector.iter(){
        let freq: &mut u32 = free_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("free_vec {:?}",free_vec);

}