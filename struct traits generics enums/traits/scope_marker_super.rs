//----------------------------------
// 
//      Scope of a Trait
//      Trait methods with same name for a type
//      Super Traits
//      Marker Traits
//      Auto Traits
// 
//----------------------------------


// Scope of a Trait
mod module_trait;

pub fn some_fn(){
    use module_trait::Basic::{PersonalInfo,Person};

    let p1 = Person{
        name: "Bob".to_string(),
        age: 40
    };

    println!("{}",p1.how_old());
}



// Trait methods with same name for a type
trait Pilot{
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

struct Human;

impl Pilot for Human{
    fn fly(&self){
        println!("This is your captain speaking");
    }
}

impl Wizard for Human{
    fn fly(&self){
        println!("I can fly due to magical powers");
    }
}

impl Human{
    fn fly(&self){
        println!("Waving arms but unable to fly");
    }
}

// SUPER TRAITS
trait Person{
    fn name(&self) -> &str;
}

trait UStudent: Person{
    fn complete_info(&self)-> (&str,u8,&str);
}

struct UniStudent{
    name_std: String,
    age: u8,
    university: String
}

impl UStudent for UniStudent{
    fn complete_info(&self) -> (&str,u8,&str){
        (
            &self.name(),
            self.age,
            &self.university
        )
    }
}

impl Person for UniStudent{
    fn name(&self)-> &str{
        &self.name_std
    }
}


// MARKER TRAIT => trait that does not require any methods to be implemented
trait SomeProperties: Clone + PartialEq + Default{}

#[derive(Default,Clone,PartialEq)]
struct Student{
    name:String,
    age:u8,
    nationality: String
}

impl SomeProperties for Student{

}

// AUTO TRAITS => Automatically implemented for a type if all of its members also implement
// the trait

// here Default trait is auto implemented for struct customer since
// its member String and u8 already have this trait implemented
#[derive(Default)]
struct Customer{
    name: String,
    age: u8,
}

fn main(){
    // Scope of a Trait
    some_fn();

    // Trait methods with same name for a type
    let person = Human;
    person.fly(); // Human structs own fly function should be invoked

    Pilot::fly(&person);
    Wizard::fly(&person);


    // super traits
    let std = UniStudent{
        name_std: "Bob".to_string(),
        age: 21,
        university:"Cornell".to_string()
    };

    println!("{:#?}",std.complete_info());


    

}