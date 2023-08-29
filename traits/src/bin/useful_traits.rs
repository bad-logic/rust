//-----------------------------
// 
// 
//      PartialEq Trait
//      PartialOrd Trait
// 
// 
//-----------------------------

use std::cmp::Ordering;

// #[derive(PartialEq, PartialOrd, Clone)]
#[derive(Clone)]
struct Person{
    name:String,
    age: u32,
    earning: u32,
    saving: u32
}

impl PartialEq for Person{
    fn eq(&self, other:&Person) -> bool{
        self.age == other.age
    }
}

impl PartialOrd for Person{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>{
        self.earning.partial_cmp(&other.earning)
    }
}

fn main(){
    let bob = Person{
        name: "Bob".to_owned(),
        age: 25,
        earning: 55_000,
        saving: 15_000
    };

    let mut bob_clone = bob.clone();
    bob_clone.age = 66;
    bob_clone.earning = 40_000;
    println!("{}",bob == bob_clone);
    println!("{}",bob >= bob_clone);
    println!("{}",bob <= bob_clone);
}